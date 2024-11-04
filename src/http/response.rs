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
    pub fn construct(&mut self, request_headers: &HashMap<String, String>) -> Vec<u8> {
        // let version = self.version;

        // let status_line = format!(
        //     "{version} {status_code} {status_text}",
        //     version = self.version,
        //     status_code = self.status_code,
        //     status_text = self.status_text
        // );

        let status_line = format!(
            "{} {} {}\r\n",
            self.version, self.status_code, self.status_text
        );

        // Determine compression type from Accept-Encoding
        let compression_type = CompressionType::from_headers(request_headers);
        println!("{} {}", self.body, self.body.len());
        // Compress body if needed
        let body_bytes = compression_type
            .encode(self.body.as_bytes())
            .unwrap_or_else(|_| self.body.as_bytes().to_vec());

        // let body = String::from_utf8(body_bytes).expect("Error occurred");
        // println!("{:?}", body);
        self.headers
            .entry(String::from("Content-Type"))
            .or_insert(String::from("text/plain"));

        self.headers
            .entry(String::from("Content-Length"))
            .or_insert(format!("{}", body_bytes.len()));

        if compression_type != CompressionType::None {
            self.headers.insert(
                String::from("Content-Encoding"),
                String::from(compression_type.as_str()),
            );
        }

        let mut headers = String::new();

        for key in self.headers.keys() {
            headers.push_str(
                format!(
                    "{key}: {value}\r\n",
                    key = key,
                    value = self.headers.get(key).unwrap()
                )
                .as_str(),
            )
        }
        headers.push_str("\r\n"); // Empty line after headers

        // format!("{}\r\n{}\r\n{}", status_line, header_str, body);

        // Combine everything into bytes
        let mut response = Vec::new();
        response.extend_from_slice(status_line.as_bytes());
        response.extend_from_slice(headers.as_bytes());
        response.extend_from_slice(&body_bytes); // Add compressed body bytes directly

        response
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
