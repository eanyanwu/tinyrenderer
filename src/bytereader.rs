use std::io;
use std::io::prelude::*;

pub struct ByteReader<'a> {
    inner: &'a[u8],
    offset: usize,
}

impl ByteReader<'_> {
    /// Create a new `ByteReader`
    pub fn new<'a>(b: &'a[u8]) -> ByteReader<'a> {
        ByteReader {
            inner: b,
            offset: 0
        }
    }

    pub fn read<'a>(&'a mut self, count: usize) -> Result<&'a[u8], ByteReaderError> {
        let bytes = self.peek(count)?;
        
        self.offset += count;

        Ok(bytes)
    }

    /// Peek at the contents of the next `count` bytes. 
    /// The offset is left unchanged
    pub fn peek<'a>(&'a self, count: usize) -> Result<&'a[u8], ByteReaderError> {
        let end = self.offset + count;

        match self.inner.get(self.offset..end) {
            Some(x) => Ok(x),
            None => Err(ByteReaderError::OutOfBounds)
        }
    }
}

pub enum ByteReaderError {
    OutOfBounds
}