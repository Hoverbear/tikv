// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.


pub mod bytes;
pub mod number;

use std::io;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::error;
use protobuf;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "io error {:?}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "protobuf error {:?}", _0)]
    Protobuf(#[cause] protobuf::ProtobufError),
    #[fail(display = "bad format key(length)")]
    KeyLength,
    #[fail(display = "bad format key(padding)")]
    KeyPadding,
    #[fail(display = "key not found")]
    KeyNotFound,
    #[fail(display = "{}", _0)]
    InvalidDataType(String),
    #[fail(display = "encoding failed")]
    Encoding(#[cause] Utf8Error),
    #[fail(display = "unknown error {:?}", _0)]
    Other(#[cause] Box<error::Error + Sync + Send>),
}

impl Error {
    pub fn maybe_clone(&self) -> Option<Error> {
        match *self {
            Error::KeyLength => Some(Error::KeyLength),
            Error::KeyPadding => Some(Error::KeyPadding),
            Error::KeyNotFound => Some(Error::KeyNotFound),
            Error::InvalidDataType(ref r) => Some(Error::InvalidDataType(r.clone())),
            Error::Encoding(e) => Some(Error::Encoding(e)),
            Error::Protobuf(_) | Error::Io(_) | Error::Other(_) => None,
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        err.utf8_error().into()
    }
}
