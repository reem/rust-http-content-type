use std::io::IoResult;
use std::collections::hashmap::HashMap;

use super::{get_file_reader, get_file_writer};

// Generate response/mimes/mod.rs
pub fn generate(list: Path, module: Path) -> IoResult<()> {
    let mut reader = get_file_reader(list);
    let mut writer = get_file_writer(module);

    try!(writer.write(b"\
// This is an automatically generated file.

use http::headers::content_type::MediaType;

pub fn get_generated_content_type(ext: &str) -> Option<MediaType> {
    match ext {"));

    /* Generated snippets will look like:
    "json" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "json".to_string(),
        parameters: vec![]
    }),
    */

    let re = regex!(r"(\S*)/(\S*)\s*(\S*)");
    let mimes = reader.read_to_string().unwrap();
    let mut seen = HashMap::new(); // Avoid duplicates
    for line in mimes.as_slice().lines() {
        // Ignore commented lines
        if line.char_at(0) == '#' { continue }

        // Everything should match
        let caps = re.captures(line).unwrap();
        let (ext, type_, subtype) = (caps.at(3), caps.at(1), caps.at(2));
        if !seen.contains_key(&ext) {
            try!(write!(writer, "
        \"{}\" => Some(MediaType {{
            type_: \"{}\".to_string(),
            subtype: \"{}\".to_string(),
            parameters: vec![]
        }}),", ext, type_, subtype));

            seen.insert(ext, true);
        }
    }

    writer.write(b"
        _ => None\n    }\n}\n")
}
