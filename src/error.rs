use hyper;
use serde_json;
use std::error::Error;
use std::fmt;
use std::io;
use std::result;
use glib;


pub type Result<T> = result::Result<T, XMZModTouchGuiError>;

#[derive(Debug)]
pub enum XMZModTouchGuiError {
    Glib(glib::Error),
    Hyper(hyper::Error),
    IoError(io::Error),
    SerdeJson(serde_json::Error),
    Unknown,
}

impl fmt::Display for XMZModTouchGuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XMZModTouchGuiError::Glib(ref err) => err.fmt(f),
            XMZModTouchGuiError::Hyper(ref err) => err.fmt(f),
            XMZModTouchGuiError::IoError(ref err) => err.fmt(f),
            XMZModTouchGuiError::SerdeJson(ref err) => err.fmt(f),
            XMZModTouchGuiError::Unknown => write!(f, "A Unknown error happens."),
        }
    }
}

impl Error for XMZModTouchGuiError {
    fn description(&self) -> &str {
        match *self {
            XMZModTouchGuiError::Glib(ref err) => err.description(),
            XMZModTouchGuiError::Hyper(ref err) => err.description(),
            XMZModTouchGuiError::IoError(ref err) => err.description(),
            XMZModTouchGuiError::SerdeJson(ref err) => err.description(),
            XMZModTouchGuiError::Unknown => "Unknown",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            XMZModTouchGuiError::Glib(ref err) => Some(err),
            XMZModTouchGuiError::Hyper(ref err) => Some(err),
            XMZModTouchGuiError::IoError(ref err) => Some(err),
            XMZModTouchGuiError::SerdeJson(ref err) => Some(err),
            XMZModTouchGuiError::Unknown => None,
        }
    }
}

impl From<glib::Error> for XMZModTouchGuiError {
    fn from(err: glib::Error) -> XMZModTouchGuiError {
        XMZModTouchGuiError::Glib(err)
    }
}

impl From<hyper::Error> for XMZModTouchGuiError {
    fn from(err: hyper::Error) -> XMZModTouchGuiError {
        XMZModTouchGuiError::Hyper(err)
    }
}

impl From<io::Error> for XMZModTouchGuiError {
    fn from(err: io::Error) -> XMZModTouchGuiError {
        XMZModTouchGuiError::IoError(err)
    }
}

impl From<serde_json::Error> for XMZModTouchGuiError {
    fn from(err: serde_json::Error) -> XMZModTouchGuiError {
        XMZModTouchGuiError::SerdeJson(err)
    }
}
