use std::io::{Read, Write, Result, Error, ErrorKind};

/// `FailingIoStream` mocks a stream which will fail upon read or write
///
/// # Examples
///
/// ```
/// use std::io::{Cursor, Read};
///
/// struct CountIo {}
///
/// impl CountIo {
///     fn read_data(&self, r: &mut Read) -> usize {
///         let mut count: usize = 0;
///         let mut retries = 3;
///
///         loop {
///             let mut buffer = [0; 5];
///             match r.read(&mut buffer) {
///                 Err(_) => {
///                     if retries == 0 { break; }
///                     retries -= 1;
///                 },
///                 Ok(0) => break,
///                 Ok(n) => count += n,
///             }
///         }
///         count
///     }
/// }
///
/// #[test]
/// fn test_io_retries() {
///     let mut c = Cursor::new(&b"1234"[..])
///             .chain(FailingIoStream::new(ErrorKind::Other, "Failing", 3))
///             .chain(Cursor::new(&b"5678"[..]));
///
///     let sut = CountIo {};
///     // this will fail unless read_data performs at least 3 retries on I/O errors
///     assert_eq!(8, sut.read_data(&mut c));
/// }
/// ```
#[derive(Clone)]
pub struct FailingIoStream {
    kind: ErrorKind,
    message: &'static str,
    repeat_count: i32,
}

impl FailingIoStream {
    /// Creates a FailingIoStream
    ///
    /// When `read` or `write` is called, it will return an error `repeat_count` times.
    /// `kind` and `message` can be specified to define the exact error.
    pub fn new(kind: ErrorKind, message: &'static str, repeat_count: i32) -> FailingIoStream {
        FailingIoStream { kind: kind, message: message, repeat_count: repeat_count, }
    }

    fn error(&mut self) -> Result<usize> {
        if self.repeat_count == 0 {
            return Ok(0)
        }
        else {
            if self.repeat_count > 0 {
                self.repeat_count -= 1;
            }
            Err(Error::new(self.kind, self.message))
        }
    }
}

impl Read for FailingIoStream {
    fn read(&mut self, _: &mut [u8]) -> Result<usize> {
        self.error()
    }
}

impl Write for FailingIoStream {
    fn write(&mut self, _: &[u8]) -> Result<usize> {
        self.error()
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::FailingIoStream;
    use std::io::{Cursor, Read, Write, ErrorKind};
    use std::error::Error;

    #[test]
    fn test_failing_mock_stream_read() {
        let mut s = FailingIoStream::new(ErrorKind::BrokenPipe, "The dog ate the ethernet cable", 1);
        let mut v = [0; 4];
        let error = s.read(v.as_mut()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::BrokenPipe);
        assert_eq!(error.description(), "The dog ate the ethernet cable");
        // after a single error, it will return Ok(0)
        assert_eq!(s.read(v.as_mut()).unwrap(), 0);
    }

    #[test]
    fn test_failing_mock_stream_chain() {
        let mut c = Cursor::new(&b"abcd"[..])
                .chain(FailingIoStream::new(ErrorKind::Other, "Failing", -1));

        let mut v = [0; 8];
        assert_eq!(c.read(v.as_mut()).unwrap(), 4);
        assert_eq!(c.read(v.as_mut()).unwrap_err().kind(), ErrorKind::Other);
        assert_eq!(c.read(v.as_mut()).unwrap_err().kind(), ErrorKind::Other);
    }

    #[test]
    fn test_failing_mock_stream_chain_interrupted() {
        let mut c = Cursor::new(&b"abcd"[..])
                .chain(FailingIoStream::new(ErrorKind::Interrupted, "Interrupted", 5))
                .chain(Cursor::new(&b"ABCD"[..]));

        let mut v = [0; 8];
        c.read_exact(v.as_mut()).unwrap();
        assert_eq!(v, [0x61, 0x62, 0x63, 0x64, 0x41, 0x42, 0x43, 0x44]);
        assert_eq!(c.read(v.as_mut()).unwrap(), 0);
    }

    #[test]
    fn test_failing_mock_stream_write() {
        let mut s = FailingIoStream::new(ErrorKind::PermissionDenied, "Access denied", -1);
        let error = s.write("abcd".as_bytes()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::PermissionDenied);
        assert_eq!(error.description(), "Access denied");
        // it will keep failing
        assert!(s.write("abcd".as_bytes()).is_err());
    }
}
