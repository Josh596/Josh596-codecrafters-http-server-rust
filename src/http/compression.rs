use std::{collections::HashMap, io::Write};

use flate2::{
    write::{GzDecoder, GzEncoder},
    Compression,
};

#[derive(Debug, PartialEq, Eq)]
pub enum CompressionType {
    Gzip,
    // Deflate,
    None,
}

impl CompressionType {
    pub fn from_str(encoding: &str) -> Self {
        match encoding.to_lowercase().as_str() {
            "gzip" => CompressionType::Gzip,
            // "deflate" => CompressionType::Deflate,
            _ => CompressionType::None,
        }
    }

    pub fn from_headers(headers: &HashMap<String, String>) -> Self {
        let accepeted_encodings = headers.get("Accept-Encoding");

        if let Some(encodings) = accepeted_encodings {
            let encodings: Vec<String> = encodings
                .split(",")
                .map(|encoding| encoding.trim().to_string())
                .collect();
            if encodings.contains(&String::from("gzip")) {
                return Self::Gzip;
            }

            Self::None
        } else {
            CompressionType::None
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Gzip => "gzip",
            // CompressionType::Deflate => "deflate",
            Self::None => "identity",
        }
    }

    pub fn encode(&self, content: &[u8]) -> Result<Vec<u8>, String> {
        match self {
            Self::None => Ok(content.to_vec()),
            Self::Gzip => {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder
                    .write_all(&content)
                    .map_err(|e| format!("Gzip compression failed: {}", e))?;

                encoder
                    .finish()
                    .map_err(|e| format!("Gzip compression failed: {}", e))
            }
        }
    }

    pub fn decode(&self, content: &[u8]) -> Result<Vec<u8>, String> {
        match self {
            Self::None => Ok(content.to_vec()),
            Self::Gzip => {
                let writer = Vec::new();
                let mut decoder = GzDecoder::new(writer);

                decoder
                    .write_all(&content)
                    .map_err(|e| format!("Gzip decompression failed: {}", e))?;

                decoder
                    .finish()
                    .map_err(|e| format!("Gzip decompression failed: {}", e))
            }
        }
    }
}
