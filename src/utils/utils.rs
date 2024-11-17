pub fn to_resp_bulk_string(s: String) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
}
