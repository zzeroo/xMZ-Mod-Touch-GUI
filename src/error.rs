use hyper;
use serde_json;
use std::error::Error;
use std::fmt;
use std::io;
use std::result;
use glib;


pub type Result<T> = result::Result<T, GuiError>;

#[derive(Debug)]
pub enum GuiError {
    Glib(glib::Error),
    Hyper(hyper::Error),
    IoError(io::Error),
    SerdeJson(serde_json::Error),
    Unknown,
}

impl fmt::Display for GuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GuiError::Glib(ref err) => err.fmt(f),
            GuiError::Hyper(ref err) => err.fmt(f),
            GuiError::IoError(ref err) => err.fmt(f),
            GuiError::SerdeJson(ref err) => err.fmt(f),
            GuiError::Unknown => write!(f, "A Unknown error happens."),
        }
    }
}

impl Error for GuiError {
    fn description(&self) -> &str {
        match *self {
            GuiError::Glib(ref err) => err.description(),
            GuiError::Hyper(ref err) => err.description(),
            GuiError::IoError(ref err) => err.description(),
            GuiError::SerdeJson(ref err) => err.description(),
            GuiError::Unknown => "Unknown",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GuiError::Glib(ref err) => Some(err),
            GuiError::Hyper(ref err) => Some(err),
            GuiError::IoError(ref err) => Some(err),
            GuiError::SerdeJson(ref err) => Some(err),
            GuiError::Unknown => None,
        }
    }
}

impl From<glib::Error> for GuiError {
    fn from(err: glib::Error) -> GuiError {
        GuiError::Glib(err)
    }
}

impl From<hyper::Error> for GuiError {
    fn from(err: hyper::Error) -> GuiError {
        GuiError::Hyper(err)
    }
}

impl From<io::Error> for GuiError {
    fn from(err: io::Error) -> GuiError {
        GuiError::IoError(err)
    }
}

impl From<serde_json::Error> for GuiError {
    fn from(err: serde_json::Error) -> GuiError {
        GuiError::SerdeJson(err)
    }
}
