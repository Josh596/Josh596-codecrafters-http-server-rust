use std::collections::HashMap;

use crate::http::compression::CompressionType;

pub struct HTTPResponse {
    pub status_code: u32,
    pub status_text: String,
    pub headers: HashMap<String, String>, // Should be a HashMap
    pub body: String,
    pub version: String,
}

impl HTTPResponse {
    pub fn construct(&mut self, request_headers: &HashMap<String, String>) -> String {
        // let version = self.version;

        let status_line = format!(
            "{version} {status_code} {status_text}",
            version = self.version,
            status_code = self.status_code,
            status_text = self.status_text
        );

        // Determine compression type from Accept-Encoding
        let compression_type = request_headers
            .get("Accept-Encoding")
            .map_or(CompressionType::None, |accept_encoding| {
                CompressionType::from_str(accept_encoding)
            });

        // Compress body if needed
        let body_bytes = compression_type
            .encode(self.body.as_bytes())
            .unwrap_or_else(|_| self.body.as_bytes().to_vec());

        self.headers
            .entry(String::from("Content-Type"))
            .or_insert(String::from("text/plain"));

        self.headers
            .entry(String::from("Content-Length"))
            .or_insert(format!("{}", self.body.len()));

        if compression_type != CompressionType::None {
            self.headers.insert(
                String::from("Content-Encoding"),
                String::from(compression_type.as_str()),
            );
        }

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
            "{}\r\n{}\r\n{}",
            status_line,
            header_str,
            String::from_utf8_lossy(&body_bytes)
        )

        // let res = format!("{self.ver}");
    }

    pub fn error_404() -> Self {
        HTTPResponse {
            status_code: 404,
            status_text: String::from("Not Found"),
            headers: HashMap::new(),
            body: "".to_string(),
            version: String::from("HTTP/1.1"),
        }
    }

    pub fn error_405() -> Self {
        HTTPResponse {
            status_code: 405,
            status_text: String::from("Method Not Allowed"),
            headers: HashMap::new(),
            body: "".to_string(),
            version: String::from("HTTP/1.1"),
        }
    }

    pub fn error_500() -> Self {
        HTTPResponse {
            status_code: 500,
            status_text: String::from("Internal Server Error"),
            headers: HashMap::new(),
            body: "".to_string(),
            version: String::from("HTTP/1.1"),
        }
    }
}
