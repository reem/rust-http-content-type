//! Get content-types for rust-http from file extensions
//!
//! Simple Example:
//! ```rust
//! req.headers.content_type = get_content_type("txt");
//! ```

#![deny(warnings, missing_docs)]
#![feature(phase)]

#[phase(plugin)] extern crate generator;
#[phase(plugin)] extern crate phf_mac;

extern crate http;
extern crate phf;

use http::headers::content_type::MediaType;

static MIMES: phf::Map<&'static str, RawMediaType>
    = mime_map!("http://svn.apache.org/repos/asf/httpd/httpd/trunk/docs/conf/mime.types");

// Generator uses RawMediaType unhygiencally. We could create another
// crate and put it there, but sticking the definition here is much
// easier.
//
// That said, this is a hack to avoid having to link against generator
// after compile-time.
struct RawMediaType {
    pub type_: &'static str,
    pub subtype: &'static str
}

/// Get the rust-http MediaType associated with this extension.
pub fn get_content_type(ext: &str) -> Option<MediaType> {
    MIMES.get(ext)
         .map(to_media_type)
}

fn to_media_type(raw: &RawMediaType) -> MediaType {
    MediaType {
        type_: raw.type_.to_string(),
        subtype: raw.subtype.to_string(),
        parameters: vec![]
    }
}

#[test]
fn test_basic() {
    assert_eq!(get_content_type("flv").unwrap(),
               MediaType { type_: "video".to_string(), subtype: "x-flv".to_string(), parameters: vec![] });
}

