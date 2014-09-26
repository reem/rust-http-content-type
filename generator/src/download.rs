use http::client::RequestWriter;
use http::method::Get;
use std::collections::HashSet;
use url::Url;

// Return a string containing the text at the given url, or an error message
pub fn download_mimes(url: &str) -> Result<String, String> {
    let parsed_url = match Url::parse(url) {
        Ok(u)  => u,
        Err(e) => return Err(format!("Unable to parse url: {}", e))
    };
    let request: RequestWriter = match RequestWriter::new(Get, parsed_url) {
        Ok(request) => request,
        Err(e)      => return Err(format!("Unable to create http request: {}", e.desc)),
    };
    let mut response = match request.read_response() {
        Ok(response) => response,
        Err((_, e))  => return Err(format!("HTTP request failed: {}", e.desc))
    };
    match response.read_to_string() {
        Ok(s)  => Ok(s),
        Err(e) => Err(format!("Error while reading response: {}", e.desc))
    }
}

// Return a vector of tuples containing the necessary information to generate the MediaTypes
// The order is (ext, type_, subtype)
// If a line does not parse, return an error message
pub fn parse_mimes(mimes: &str) -> Result<Vec<(&str, &str, &str)>, String> {
    let re = regex!(r"(\S*)/(\S*)\s*(\S*)");
    let mut seen = HashSet::new(); // Avoid duplicates
    let mut vec = Vec::with_capacity(800);

    for line in mimes.lines() {
        // Ignore 0-length or commented lines
        if line.len() == 0 || line.char_at(0) == '#' { continue }

        let caps = match re.captures(line) {
            Some(captures) => captures,
            None           => return Err(format!("'{}' does not match the regular expression pattern", line))
        };

        let (ext, type_, subtype) = (caps.at(3), caps.at(1), caps.at(2));
        if !seen.contains(&ext) {
            vec.push((ext, type_, subtype));
            seen.insert(ext);
        }
    }

    Ok(vec)
}
