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

mod endpoint;
mod metrics;
mod dag;
mod statistics;
pub mod select;
pub mod codec;

use std::result;
use std::error;

use kvproto::kvrpcpb::LockInfo;
use kvproto::errorpb;

use storage::{engine, mvcc, txn};
use util::time::Instant;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "region {:?}", _0)]
    Region(#[cause] errorpb::Error),
    #[fail(display = "locked {:?}", _0)]
    Locked(LockInfo),
    #[fail(display = "request is outdated")]
    Outdated { deadline: Instant, now: Instant, tag: &'static str },
    #[fail(display = "running queue is full")]
    Full(usize),
    #[fail(display = "unknown error {:?}", _0)]
    Other(#[cause] Box<error::Error + Send + Sync>),
}

pub type Result<T> = result::Result<T, Error>;

impl From<engine::Error> for Error {
    fn from(e: engine::Error) -> Error {
        match e {
            engine::Error::Request(e) => Error::Region(e),
            _ => Error::Other(box e),
        }
    }
}

impl From<txn::Error> for Error {
    fn from(e: txn::Error) -> Error {
        match e {
            txn::Error::Mvcc(mvcc::Error::KeyIsLocked {
                primary,
                ts,
                key,
                ttl,
            }) => {
                let mut info = LockInfo::new();
                info.set_primary_lock(primary);
                info.set_lock_version(ts);
                info.set_key(key);
                info.set_lock_ttl(ttl);
                Error::Locked(info)
            }
            _ => Error::Other(box e),
        }
    }
}

pub use self::endpoint::{CopRequestStatistics, CopSender, Host as EndPointHost, RequestTask,
                         Task as EndPointTask, REQ_TYPE_DAG, REQ_TYPE_INDEX, REQ_TYPE_SELECT,
                         SINGLE_GROUP};
