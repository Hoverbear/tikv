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

mod store;
mod scheduler;
mod latch;

use std::error;
use std::io::Error as IoError;

pub use self::scheduler::{Msg, Scheduler, GC_BATCH_SIZE, RESOLVE_LOCK_BATCH_SIZE};
pub use self::store::{SnapshotStore, StoreScanner};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "engine error: {:?}", _0)]
    Engine(#[cause] ::storage::engine::Error),
    #[fail(display = "codec error: {:?}", _0)]
    Codec(#[cause] ::util::codec::Error),
    #[fail(display = "protobuf error: {:?}", _0)]
    ProtoBuf(#[cause] ::protobuf::error::ProtobufError),
    #[fail(display = "mvcc error: {:?}", _0)]
    Mvcc(#[cause] ::storage::mvcc::Error),
    #[fail(display = "other error: {:?}", _0)]
    Other(#[cause] Box<error::Error + Sync + Send>),
    #[fail(display = "io error: {:?}", _0)]
    Io(#[cause] IoError),
    #[fail(display = "Invalid transaction tso with start_ts:{},commit_ts:{}",
                    start_ts,
                    commit_ts)]
    InvalidTxnTso { start_ts: u64, commit_ts: u64 },
}

impl Error {
    pub fn maybe_clone(&self) -> Option<Error> {
        match *self {
            Error::Engine(ref e) => e.maybe_clone().map(Error::Engine),
            Error::Codec(ref e) => e.maybe_clone().map(Error::Codec),
            Error::Mvcc(ref e) => e.maybe_clone().map(Error::Mvcc),
            Error::InvalidTxnTso {
                start_ts,
                commit_ts,
            } => Some(Error::InvalidTxnTso {
                start_ts: start_ts,
                commit_ts: commit_ts,
            }),
            Error::Other(_) | Error::ProtoBuf(_) | Error::Io(_) => None,
        }
    }
}
