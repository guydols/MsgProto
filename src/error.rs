use core::str;
use std::ffi::OsString;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	/// For starter, to remove as code matures.
	#[error("Generic error: {0}")]
	Generic(String),

	/// For starter, to remove as code matures.
	#[error("Static error: {0}")]
	Static(&'static str),

	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error(transparent)]
	Time(#[from] std::time::SystemTimeError),
	
	#[error(transparent)]
	Bitcode(#[from] bitcode::Error),

	#[error(transparent)]
	IPParse(#[from]std::net::AddrParseError),
	
	#[error(transparent)]
	IntParse(#[from]std::num::ParseIntError),
	
	#[error("Invalid path")]
	InvalidPath(OsString),

	#[error(transparent)]
	UTF8(#[from]std::str::Utf8Error),

}