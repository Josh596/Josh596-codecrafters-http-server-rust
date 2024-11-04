use std::io::Write;

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

    pub fn as_str(&self) -> &str {
        match self {
            CompressionType::Gzip => "gzip",
            // CompressionType::Deflate => "deflate",
            CompressionType::None => "identity",
        }
    }

    pub fn encode(&self, content: &[u8]) -> Result<Vec<u8>, String> {
        match self {
            CompressionType::None => Ok(content.to_vec()),
            CompressionType::Gzip => {
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
            CompressionType::None => Ok(content.to_vec()),
            CompressionType::Gzip => {
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
