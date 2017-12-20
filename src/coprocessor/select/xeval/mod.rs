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


pub mod evaluator;
mod builtin_math;

use util::codec;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "codec failed")]
    Codec(#[cause] codec::Error),
    #[fail(display = "{}", _0)]
    Expr(String),
    #[fail(display = "{}", _0)]
    Eval(String)
}

use std::result;
pub type Result<T> = result::Result<T, Error>;

pub use self::evaluator::{EvalContext, Evaluator};
