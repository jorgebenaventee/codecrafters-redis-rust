pub fn to_resp_bulk_string(s: String) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
}

pub fn to_resp_array_string(strs: Vec<String>) -> String {
    let mut resp = String::new();
    let len = strs.len();
    resp.push_str(&format!("*{}\r\n", len));
    for s in strs {
        resp.push_str(&to_resp_bulk_string(s));
    }
    resp
}
