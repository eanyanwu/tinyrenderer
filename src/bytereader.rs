use std::error;
use std::fmt;

pub struct ByteReader<'a> {
    inner: &'a[u8],
    offset: usize,
}

impl<'a> ByteReader<'a> {
    /// Create a new `ByteReader`
    pub fn new(b: &'a[u8]) -> ByteReader<'a> {
        ByteReader {
            inner: b,
            offset: 0
        }
    }

    pub fn read(&mut self, count: usize) -> Result<&'a[u8], ByteReaderError> {
        let bytes = self.peek(count)?;
        
        self.seek(self.offset + count);

        Ok(bytes)
    }

    /// Peek at the contents of the next `count` bytes. 
    /// The offset is left unchanged
    pub fn peek(&self, count: usize) -> Result<&'a[u8], ByteReaderError> {
        let end = self.offset + count;

        match self.inner.get(self.offset..end) {
            Some(x) => Ok(x),
            None => Err(ByteReaderError::OutOfBounds(OutOfBoundsError::new(self.inner.len(), end)))
        }
    }

    /// Sets the offset to `offset`
    /// If the new offset exceeds the array bounds, an `OutOfBounds` error will be returned
    pub fn seek(&mut self, offset: usize) -> Result<(), ByteReaderError> {
        if offset >= self.inner.len() {
            Err(ByteReaderError::OutOfBounds(OutOfBoundsError::new(self.inner.len(), offset)))
        }
        else{
            self.offset = offset;
            Ok(())
        }
    }

    /// Attempts to match the subsequent bytes to `expected` 
    /// A successful match will result in moving the byte reader forward
    /// A failed match will keep our position unchanged.
    pub fn try_accept(&mut self, expected: &[u8]) -> bool {
        let current_offset = self.offset;

        if let Ok(_) = self.accept(expected) {
            true
        }
        else {
            // Seek back to our previous position
            // Safe to unwrap, as we are seeking back to a known position
            self.seek(current_offset).unwrap(); 
            false
        }
    }

    /// Matches the subsequent bytes to `expected` 
    /// If the match is not successful, an `UnexpectedByte` error is returned.
    /// An `OutOfBounds` error may be returned if reading results in going past the 
    /// EOF
    pub fn accept(&mut self, expected: &[u8]) -> Result<(), ByteReaderError> {
        let count = expected.len();
        let bytes = self.read(count)?;

        if expected == bytes {
            Ok(())
        }
        else {
            Err(ByteReaderError::UnexpectedValue(UnexpectedValueError::new(expected, bytes)))
        }
    }

    /// Get the current offset 
    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[derive(Debug)]
pub enum ByteReaderError {
    OutOfBounds(OutOfBoundsError),
    UnexpectedValue(UnexpectedValueError),
}

#[derive(Debug)]
pub struct OutOfBoundsError { bounds: usize, offset: usize }
#[derive(Debug)]
pub struct UnexpectedValueError { expected: Vec<u8>, actual: Vec<u8> }

impl OutOfBoundsError {
    pub fn new(bounds: usize, offset: usize) -> Self {
        OutOfBoundsError { bounds, offset }
    }
}

impl UnexpectedValueError {
    pub fn new(expected: &[u8], actual: &[u8]) -> Self {
        UnexpectedValueError {
            expected: expected.to_vec(),
            actual: actual.to_vec(),
        }
    }
}

impl fmt::Display for ByteReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ByteReaderError::OutOfBounds(x) => write!(f, "OutOfBounds: index {} is out of bounds {}", x.offset, x.bounds),
            ByteReaderError::UnexpectedValue(x) => write!(f, "UnexpectedValue: expected {:?}, got {:?}", x.expected, x.actual),
        }
    }
}

impl error::Error for ByteReaderError {}

#[cfg(test)]
mod unit_tests {
    use crate::bytereader::{
        ByteReaderError,
        OutOfBoundsError,
        ByteReader};

    #[test]
    fn test_read_empty_slice() -> Result<(), ()> {
        let mut reader = ByteReader::new(&[]);
        
        let res = reader.read(1);

        assert!(res.is_err());

        match res.err() {
            Some(ByteReaderError::OutOfBounds(_)) => Ok(()),
            _ => Err(())
        }
    }

    #[test]
    fn test_read_out_of_bounds_with_correct_error() -> Result<(), ()> {
        let mut reader = ByteReader::new(&[1,2,3]);

        let res = reader.read(4);

        assert!(res.is_err());
        
        if let ByteReaderError::OutOfBounds(OutOfBoundsError { bounds: x, offset: y }) = res.err().unwrap() {
            assert!(x == 3);
            assert!(y == 4);
            Ok(())
        }
        else {
            Err(())
        }
    }

    #[test]
    fn test_peak_does_not_move_offset() {
        let reader = ByteReader::new(&[1,2,3]);

        let res = reader.peek(3);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &[1,2,3]);
        assert_eq!(reader.offset(), 0);
    }
}