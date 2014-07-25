//! Get content-types for rust-http from file extensions

extern crate http;

pub use get_content_type = self::mimes::get_generated_content_type;
mod mimes;

