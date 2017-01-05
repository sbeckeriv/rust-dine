use std::{convert, error, fmt};
use serde_xml;

#[derive(Debug)]
pub enum Error {
    XmlError(serde_xml::error::Error),
}

impl convert::From<serde_xml::error::Error> for Error {
    fn from(xml_error: serde_xml::error::Error) -> Self {
        Error::XmlError(xml_error)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::XmlError(ref xml) => error::Error::description(xml),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::XmlError(ref xml) => Some(xml),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::XmlError(ref xml) => write!(fmt, "xml error: {}", xml),
        }
    }
}
