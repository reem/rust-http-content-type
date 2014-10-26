//! Get content-types for rust-http from file extensions
//!
//! Simple Example:
//! ```rust
//! req.headers.content_type = get_content_type("txt");
//! ```

#![feature(phase)]

#[phase(plugin)] extern crate generator;
#[phase(plugin)] extern crate phf_mac;

extern crate http;
extern crate generator;
extern crate phf;

use generator::RawMediaType;
use http::headers::content_type::MediaType;
use phf::PhfMap;

static MIMES: PhfMap<&'static str, RawMediaType>
    = mime_map!("http://svn.apache.org/repos/asf/httpd/httpd/trunk/docs/conf/mime.types");

pub fn get_content_type(ext: &str) -> Option<MediaType> {
    MIMES.find_equiv(&ext)
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
