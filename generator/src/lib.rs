//! Compiler plugin for rust-http-content-type
//!
//! # Usage
//!
//! ```
//! let mimes: PhfMap<&'static str, RawMediaType>
//!     = mime_map!("http://svn.apache.org/repos/asf/httpd/httpd/trunk/docs/conf/mime.types");
//! ```

#![crate_type = "dylib"]
#![license = "MIT"]

#![feature(macro_rules, plugin_registrar, phase)]

extern crate http;
extern crate phf_mac;
extern crate regex;
#[phase(plugin)] extern crate regex_macros;
extern crate rustc;
extern crate syntax;
extern crate url;

use phf_mac::util as phf;
use phf_mac::util::{Entry, KeyStr};
use rustc::plugin::Registry;
use syntax::{ast, codemap, parse};
use syntax::ast::{CookedStr, LitStr};
use syntax::codemap::Spanned;
use syntax::ext::base::{DummyResult, ExtCtxt, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::token::{mod, Eof};
use syntax::ptr::P;

use download::{download_mimes, parse_mimes};

mod download;

pub struct RawMediaType {
    pub type_: &'static str,
    pub subtype: &'static str
}


#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("mime_map", expand);
}

// See http://doc.rust-lang.org/nightly/syntax/ext/build/trait.AstBuilder.html for documentation
// about the functions used to build the AST.
fn expand(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult + 'static> {
    // Try to parse the arguments of the macro
    let url = match parse_arguments(cx, sp, tts) {
        Ok(x) => { x }
        Err((span, msg)) => {
            cx.span_err(span, msg);
            return DummyResult::any(span);
        }
    };

    // Download the mimes from the given url
    let mime_text = match download_mimes(url.node.as_slice()) {
        Ok(v)  => v,
        Err(m) => {
            cx.span_err(url.span, m.as_slice());
            return DummyResult::any(url.span);
        }
    };

    // Parse the mimes using regular expressions
    let parsed_mimes = match parse_mimes(mime_text.as_slice()) {
        Ok(v)  => v,
        Err(m) => {
            cx.span_err(url.span, m.as_slice());
            return DummyResult::any(url.span);
        }
    };

    // Create the entries of the PhfMap
    let entries: Vec<Entry> = parsed_mimes.into_iter()
                                          .map(|(e, t, s)| generate_entry(cx, sp, e, t, s))
                                          .collect();
    let hash = phf::generate_hash(cx, sp, entries.as_slice());

    phf::create_map(cx, sp, entries, hash)
}

// Returns an `Entry` with `ext` as key and a `RawMediaType` as value
fn generate_entry(cx: &ExtCtxt, sp: codemap::Span, ext: &str, type_: &str, subtype: &str) -> Entry {
    // The key
    let key = str_to_lit(cx, sp, ext);
    let key_contents = KeyStr(token::intern_and_get_ident(ext));

    // The raw media type
    let raw_media_type = generate_media_type(cx, sp, type_, subtype);

    // The entry in the PhfMap
    Entry {
        key: key,
        key_contents: key_contents,
        value: raw_media_type
    }
}

// Returns an expression corresponding to the declaration of a new `RawMediaType` object
fn generate_media_type(cx: &ExtCtxt, sp: codemap::Span, type_: &str, subtype: &str)
    -> P<ast::Expr>
{
    let get_field = |name, value| cx.field_imm(sp, cx.ident_of(name), str_to_lit(cx, sp, value));

    let fields = vec![get_field("type_", type_),
                      get_field("subtype", subtype)];

    let struct_name = cx.path_ident(sp, cx.ident_of("RawMediaType"));

    cx.expr_struct(sp, struct_name, fields)
}

// Transforms a `&str` into a string literal expression
fn str_to_lit(cx: &ExtCtxt, sp: codemap::Span, s: &str) -> P<ast::Expr> {
    let interned = token::intern_and_get_ident(s);
    let lit_ = LitStr(interned, CookedStr);
    cx.expr_lit(sp, lit_)
}

// We expect the only argument to be a string literal, containing the url
// In case of error, we will return a span and an error message
fn parse_arguments(cx: &mut ExtCtxt, main_sp: codemap::Span, tts: &[ast::TokenTree])
    -> Result<Spanned<String>, (codemap::Span, &'static str)>
{
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts.to_vec());

    let literal = if parser.token != Eof {
        parser.parse_lit()
    } else {
        return Err((main_sp, "Unexpected end of file"));
    };

    // The string literal
    let string = match literal.node {
        LitStr(content, _) => Spanned { node: content.get().to_string(), span: literal.span },
        _                  => return Err((main_sp, "String literal expected"))
    };

    Ok(string)
}
