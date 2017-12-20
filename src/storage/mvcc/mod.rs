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

mod reader;
mod txn;
mod lock;
mod write;
mod metrics;

use std::io;
use std::error;
pub use self::txn::{MvccTxn, MAX_TXN_WRITE_SIZE};
pub use self::reader::MvccReader;
pub use self::lock::{Lock, LockType};
pub use self::write::{Write, WriteType};
use util::escape;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "engine error: {:?}", _0)]
    Engine(#[cause] ::storage::engine::Error),
    #[fail(display = "io error: {:?}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "codec error: {:?}", _0)]
    Codec(#[cause] ::util::codec::Error),
    #[fail(display = "key is locked (backoff or cleanup) {}-{}@{} ttl {}",
                    key,
                    primary,
                    ts,
                    ttl)]
    KeyIsLocked { key: Vec<u8>, primary: Vec<u8>, ts: u64, ttl: u64},
    #[fail(display = "bad format lock data")]
    BadFormatLock,
    #[fail(display = "bad format write data")]
    BadFormatWrite,
    #[fail(display = "txn already committed @{}", commit_ts)]
    Committed { commit_ts: u64 },
    #[fail(display = "txn lock not found {}-{} key:{:?}", start_ts, commit_ts, key)]
    TxnLockNotFound { start_ts: u64, commit_ts: u64, key: Vec<u8> },
    #[fail(display = "write conflict {} with {}, key:{:?}, primary:{:?}",
            start_ts, conflict_ts, key, primary)]
    WriteConflict { start_ts: u64, conflict_ts: u64, key: Vec<u8>, primary: Vec<u8> },
    #[fail(display = "bad format key(version)")]
    KeyVersion,
    #[fail(display = "{:?}", _0)]
    Other(#[cause] Box<error::Error + Sync + Send>),
}

impl Error {
    pub fn maybe_clone(&self) -> Option<Error> {
        match *self {
            Error::Engine(ref e) => e.maybe_clone().map(Error::Engine),
            Error::Codec(ref e) => e.maybe_clone().map(Error::Codec),
            Error::KeyIsLocked {
                ref key,
                ref primary,
                ts,
                ttl,
            } => Some(Error::KeyIsLocked {
                key: key.clone(),
                primary: primary.clone(),
                ts: ts,
                ttl: ttl,
            }),
            Error::BadFormatLock => Some(Error::BadFormatLock),
            Error::BadFormatWrite => Some(Error::BadFormatWrite),
            Error::TxnLockNotFound {
                start_ts,
                commit_ts,
                ref key,
            } => Some(Error::TxnLockNotFound {
                start_ts: start_ts,
                commit_ts: commit_ts,
                key: key.to_owned(),
            }),
            Error::WriteConflict {
                start_ts,
                conflict_ts,
                ref key,
                ref primary,
            } => Some(Error::WriteConflict {
                start_ts: start_ts,
                conflict_ts: conflict_ts,
                key: key.to_owned(),
                primary: primary.to_owned(),
            }),
            Error::KeyVersion => Some(Error::KeyVersion),
            Error::Committed { commit_ts } => Some(Error::Committed {
                commit_ts: commit_ts,
            }),
            Error::Io(_) | Error::Other(_) => None,
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
