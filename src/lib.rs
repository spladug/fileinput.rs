//! Read from multiple input streams.
//!
//! A `FileInput` implements the `std::io::Read` trait and reads the contents of each file
//! specified (`-` means standard input), or standard input if none are given.
//!
//! An example that prints out all the lines in each of the two files specified:
//!
//! ```
//! use std::io::{BufRead,BufReader};
//! use fileinput::FileInput;
//!
//! let filenames = vec!["testdata/1", "testdata/2"];
//! let fileinput = FileInput::new(&filenames);
//! let mut reader = BufReader::new(fileinput);
//!
//! for line in reader.lines() {
//!     println!("{}", line.unwrap());
//! }
//! ```
use std::fs::File;
use std::io;
use std::io::{Read,stdin};
use std::borrow::Borrow;


/// A file source.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Source {
    /// Read from the process's standard in.
    Stdin,
    /// Read from the specified file.
    File(String),
}

fn make_source_vec<T>(filenames: &[T]) -> Vec<Source> where T: Borrow<str> {
    if filenames.is_empty() {
        return vec![Source::Stdin];
    }

    let mut sources = Vec::with_capacity(filenames.len());
    for filename in filenames {
        sources.push(match filename.borrow() {
            "-" => Source::Stdin,
            filename => Source::File(filename.to_string()),
        });
    }
    sources
}

struct State {
    source: Source,
    reader: Box<Read>,
}

/// A wrapper which reads from multiple streams.
pub struct FileInput {
    sources: Vec<Source>,
    state: Option<State>,
}

impl FileInput {
    /// Constructs a new `FileInput` that will read from the files specified.
    pub fn new<T>(paths: &[T]) -> FileInput where T: Borrow<str> {
        FileInput {
            sources: make_source_vec(paths),
            state: None,
        }
    }

    /// Returns the current source being read from.
    ///
    /// This function will return `None` if no reading has been done yet or all the inputs have
    /// been drained.
    pub fn source(&self) -> Option<Source> {
        self.state.as_ref().map(|s| s.source.clone())
    }

    fn open_next_file(&mut self) -> io::Result<()> {
        let next_source = self.sources.remove(0);
        let reader: Box<Read> = match &next_source {
            &Source::Stdin => Box::new(stdin()),
            &Source::File(ref path) => Box::new(try!(File::open(path))),
        };

        self.state = Some(State {
            source: next_source,
            reader: reader,
        });

        Ok(())
    }
}

impl Read for FileInput {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            if self.state.is_none() {
                if self.sources.is_empty() {
                    return Ok(0);
                }

                try!(self.open_next_file());
            }

            let bytes_read = try!(self.state.as_mut().unwrap().reader.read(buf));

            if bytes_read == 0 {
                self.state = None;
                continue;
            }

            return Ok(bytes_read);
        }
    }
}

#[cfg(test)]
mod test {
    mod source_vec {
        use super::super::{make_source_vec,Source};

        #[test]
        fn empty_list_makes_stdin() {
            let names: Vec<String> = vec![];
            let paths = make_source_vec(&names);
            assert_eq!(paths, [Source::Stdin]);
        }

        #[test]
        fn dash_makes_stdin() {
            let names = vec!["-"];
            let paths = make_source_vec(&names);
            assert_eq!(paths, [Source::Stdin]);
        }

        #[test]
        fn filename_makes_path() {
            let names = vec!["example-file"];
            let paths = make_source_vec(&names);
            assert_eq!(paths, [Source::File("example-file".to_string())]);
        }

        #[test]
        fn mixed() {
            let names = vec!["one", "two", "-", "three"];
            let paths = make_source_vec(&names);
            assert_eq!(paths, [Source::File("one".to_string()), Source::File("two".to_string()), Source::Stdin, Source::File("three".to_string())]);
        }
    }

    mod fileinput {
        use super::super::*;
        use std::io::{Read,ErrorKind,BufRead,BufReader};

        #[test]
        fn read_files() {
            let paths = vec!["testdata/1", "testdata/2"];
            let mut fileinput = FileInput::new(&paths);
            let mut buffer = String::new();

            fileinput.read_to_string(&mut buffer).unwrap();

            assert_eq!(buffer, "One.\nTwo.\nTwo.\n");
        }

        #[test]
        fn skip_empty_file() {
            let paths = vec!["testdata/1", "testdata/empty", "testdata/2"];
            let mut fileinput = FileInput::new(&paths);
            let mut buffer = String::new();

            fileinput.read_to_string(&mut buffer).unwrap();

            assert_eq!(buffer, "One.\nTwo.\nTwo.\n");
        }

        #[test]
        fn get_source() {
            let paths = vec!["testdata/1", "testdata/2"];
            let fileinput = FileInput::new(&paths);
            let mut reader = BufReader::new(fileinput);
            let mut buffer = String::new();

            assert_eq!(reader.get_ref().source(), None);
            reader.read_line(&mut buffer).unwrap();
            assert_eq!(reader.get_ref().source(), Some(Source::File("testdata/1".to_string())));
            reader.read_line(&mut buffer).unwrap();
            assert_eq!(reader.get_ref().source(), Some(Source::File("testdata/2".to_string())));
            reader.read_line(&mut buffer).unwrap();
            reader.read_line(&mut buffer).unwrap();
            assert_eq!(reader.get_ref().source(), None);
        }

        #[test]
        fn error_on_nonexistent_file() {
            let paths = vec!["testdata/NOPE"];
            let mut fileinput = FileInput::new(&paths);
            let mut buffer = String::new();
            let result = fileinput.read_to_string(&mut buffer);

            assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
        }
    }
}
