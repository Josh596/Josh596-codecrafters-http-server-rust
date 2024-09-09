use std::collections::HashMap;

pub struct HTTPResponse {
    pub status_code: u8,
    pub status_text: String,
    pub headers: HashMap<String, String>, // Should be a HashMap
    pub body: String,
    pub version: String,
}

impl HTTPResponse {
    // HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\nabc
    pub fn construct(&mut self) -> String {
        // let version = self.version;

        let status_line = format!(
            "{version} {status_code} {status_text}",
            version = self.version,
            status_code = self.status_code,
            status_text = self.status_text
        );

        self.headers
            .insert(String::from("Content-Type"), String::from("text/plain"));
        self.headers.insert(
            String::from("Content-Length"),
            format!("{}", self.body.len()),
        );
        let mut header_str = String::new();

        for key in self.headers.keys() {
            header_str.push_str(
                format!(
                    "{key}: {value}\r\n",
                    key = key,
                    value = self.headers.get(key).unwrap()
                )
                .as_str(),
            )
        }

        format!(
            "{status_line}\r\n{header}\r\n{body}",
            header = header_str,
            body = self.body
        )
        // let res = format!("{self.ver}");
    }
}
