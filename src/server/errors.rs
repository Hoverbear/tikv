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

use std::error;
use std::result;
use std::io::Error as IoError;
use std::net::AddrParseError;

use futures::Canceled;
use protobuf::ProtobufError;
use grpc::Error as GrpcError;

use util::codec::Error as CodecError;
use util::worker::ScheduleError;
use raftstore::Error as RaftServerError;
use storage::engine::Error as EngineError;
use storage::Error as StorageError;
use pd::Error as PdError;
use super::snap::Task as SnapTask;
use coprocessor::EndPointTask;

#[derive(Debug, Fail)]
pub enum Error {
    // #[fail(display = "{:?}", _0)]
    // Other(#[cause] Box<error::Error + Sync + Send>),
    // // Following is for From other errors.
    // #[fail(display = "{:?}", _0)]
    // Io(#[cause] IoError),
    // #[fail(display = "{:?}", _0)]
    // Protobuf(#[cause] ProtobufError),
    // #[fail(display = "{:?}", _0)]
    // Grpc(#[cause] GrpcError),
    // #[fail(display = "{:?}", _0)]
    // Codec(#[cause] CodecError),
    // #[fail(display = "{:?}", _0)]
    // AddrParse(#[cause] AddrParseError),
    // #[fail(display = "{:?}", _0)]
    // RaftServer(#[cause] RaftServerError),
    // #[fail(display = "{:?}", _0)]
    // Engine(#[cause] EngineError),
    // #[fail(display = "{:?}", _0)]
    // Storage(#[cause] StorageError),
    // #[fail(display = "{:?}", _0)]
    // Pd(#[cause] PdError),
    // #[fail(display = "{:?}", _0)]
    // SnapWorkerStopped(#[cause] ScheduleError<SnapTask>),
    // #[fail(display = "{:?}", _0)]
    // EndPointStopped(#[cause] ScheduleError<EndPointTask>),
    #[fail(display = "failed to poll from mpsc receiver")]
    Sink,
    #[fail(display = "{:?}", _0)]
    Canceled(#[cause] Canceled),
}


pub type Result<T> = result::Result<T, Error>;
