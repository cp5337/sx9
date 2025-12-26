//! RESP (Redis Serialization Protocol) implementation
//!
//! Implements RESP2/RESP3 protocol for Redis compatibility.

use super::{SledisError, SledisResult};

/// RESP data types
#[derive(Debug, Clone, PartialEq)]
pub enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Option<Vec<RespValue>>),
}

impl RespValue {
    /// Encode RESP value to bytes
    pub fn encode(&self) -> Vec<u8> {
        match self {
            RespValue::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            RespValue::Error(e) => format!("-{}\r\n", e).into_bytes(),
            RespValue::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            RespValue::BulkString(None) => b"$-1\r\n".to_vec(),
            RespValue::BulkString(Some(s)) => {
                format!("${}\r\n{}\r\n", s.len(), s).into_bytes()
            }
            RespValue::Array(None) => b"*-1\r\n".to_vec(),
            RespValue::Array(Some(arr)) => {
                let mut result = format!("*{}\r\n", arr.len()).into_bytes();
                for item in arr {
                    result.extend(item.encode());
                }
                result
            }
        }
    }

    /// Create OK response
    pub fn ok() -> Self {
        RespValue::SimpleString("OK".to_string())
    }

    /// Create null bulk string
    pub fn null() -> Self {
        RespValue::BulkString(None)
    }

    /// Create integer response
    pub fn int(i: i64) -> Self {
        RespValue::Integer(i)
    }

    /// Create bulk string response
    pub fn bulk(s: impl Into<String>) -> Self {
        RespValue::BulkString(Some(s.into()))
    }

    /// Create error response
    pub fn err(msg: impl Into<String>) -> Self {
        RespValue::Error(msg.into())
    }

    /// Create array response
    pub fn array(items: Vec<RespValue>) -> Self {
        RespValue::Array(Some(items))
    }
}

/// RESP parser
pub struct RespParser {
    buffer: Vec<u8>,
    pos: usize,
}

impl RespParser {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            buffer: data,
            pos: 0,
        }
    }

    /// Parse RESP value from buffer
    pub fn parse(&mut self) -> SledisResult<RespValue> {
        if self.pos >= self.buffer.len() {
            return Err(SledisError::Protocol("Unexpected end of input".to_string()));
        }

        let type_byte = self.buffer[self.pos];
        self.pos += 1;

        match type_byte {
            b'+' => self.parse_simple_string(),
            b'-' => self.parse_error(),
            b':' => self.parse_integer(),
            b'$' => self.parse_bulk_string(),
            b'*' => self.parse_array(),
            _ => Err(SledisError::Protocol(format!(
                "Unknown type byte: {}",
                type_byte as char
            ))),
        }
    }

    fn read_line(&mut self) -> SledisResult<String> {
        let start = self.pos;
        while self.pos < self.buffer.len() - 1 {
            if self.buffer[self.pos] == b'\r' && self.buffer[self.pos + 1] == b'\n' {
                let line = String::from_utf8(self.buffer[start..self.pos].to_vec())
                    .map_err(|e| SledisError::Protocol(e.to_string()))?;
                self.pos += 2; // Skip \r\n
                return Ok(line);
            }
            self.pos += 1;
        }
        Err(SledisError::Protocol("Incomplete line".to_string()))
    }

    fn parse_simple_string(&mut self) -> SledisResult<RespValue> {
        Ok(RespValue::SimpleString(self.read_line()?))
    }

    fn parse_error(&mut self) -> SledisResult<RespValue> {
        Ok(RespValue::Error(self.read_line()?))
    }

    fn parse_integer(&mut self) -> SledisResult<RespValue> {
        let line = self.read_line()?;
        let i: i64 = line
            .parse()
            .map_err(|e| SledisError::Protocol(format!("Invalid integer: {}", e)))?;
        Ok(RespValue::Integer(i))
    }

    fn parse_bulk_string(&mut self) -> SledisResult<RespValue> {
        let len_str = self.read_line()?;
        let len: i64 = len_str
            .parse()
            .map_err(|e| SledisError::Protocol(format!("Invalid length: {}", e)))?;

        if len < 0 {
            return Ok(RespValue::BulkString(None));
        }

        let len = len as usize;
        if self.pos + len + 2 > self.buffer.len() {
            return Err(SledisError::Protocol("Incomplete bulk string".to_string()));
        }

        let data = String::from_utf8(self.buffer[self.pos..self.pos + len].to_vec())
            .map_err(|e| SledisError::Protocol(e.to_string()))?;
        self.pos += len + 2; // Skip data + \r\n

        Ok(RespValue::BulkString(Some(data)))
    }

    fn parse_array(&mut self) -> SledisResult<RespValue> {
        let len_str = self.read_line()?;
        let len: i64 = len_str
            .parse()
            .map_err(|e| SledisError::Protocol(format!("Invalid array length: {}", e)))?;

        if len < 0 {
            return Ok(RespValue::Array(None));
        }

        let mut items = Vec::with_capacity(len as usize);
        for _ in 0..len {
            items.push(self.parse()?);
        }

        Ok(RespValue::Array(Some(items)))
    }
}

/// Parse inline command (space-separated)
pub fn parse_inline_command(line: &str) -> Vec<String> {
    line.split_whitespace().map(String::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple_string() {
        let value = RespValue::SimpleString("OK".to_string());
        assert_eq!(value.encode(), b"+OK\r\n");
    }

    #[test]
    fn test_encode_error() {
        let value = RespValue::Error("ERR unknown command".to_string());
        assert_eq!(value.encode(), b"-ERR unknown command\r\n");
    }

    #[test]
    fn test_encode_integer() {
        let value = RespValue::Integer(42);
        assert_eq!(value.encode(), b":42\r\n");
    }

    #[test]
    fn test_encode_bulk_string() {
        let value = RespValue::BulkString(Some("hello".to_string()));
        assert_eq!(value.encode(), b"$5\r\nhello\r\n");

        let null = RespValue::BulkString(None);
        assert_eq!(null.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_encode_array() {
        let value = RespValue::Array(Some(vec![
            RespValue::BulkString(Some("GET".to_string())),
            RespValue::BulkString(Some("key".to_string())),
        ]));
        assert_eq!(value.encode(), b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n");
    }

    #[test]
    fn test_parse_array() {
        let data = b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n".to_vec();
        let mut parser = RespParser::new(data);
        let value = parser.parse().unwrap();

        match value {
            RespValue::Array(Some(arr)) => {
                assert_eq!(arr.len(), 2);
                assert_eq!(arr[0], RespValue::BulkString(Some("GET".to_string())));
                assert_eq!(arr[1], RespValue::BulkString(Some("key".to_string())));
            }
            _ => panic!("Expected array"),
        }
    }
}
