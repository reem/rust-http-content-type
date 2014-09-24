//! Get content-types for rust-http from file extensions
//!
//! Simple Example:
//! ```rust
//! req.headers.content_type = get_content_type("txt");
//! ```

#![feature(macro_rules, phase)]

extern crate http;

#[phase(plugin)]
extern crate generator;

use http::headers::content_type::MediaType;

pub fn get_content_type(ext: &str) -> Option<MediaType> {
    match_mimes!(ext, "http://svn.apache.org/repos/asf/httpd/httpd/trunk/docs/conf/mime.types")
}

#[test]
fn test_basic() {
    assert_eq!(get_content_type("flv").unwrap(),
               MediaType { type_: "video".to_string(), subtype: "x-flv".to_string(), parameters: vec![] });
}
