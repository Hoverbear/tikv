// Copyright 2016 TiKV Project Authors.
#![feature(test)]

extern crate test;

mod channel;
mod coprocessor;
mod raftkv;
mod serialization;
mod storage;
mod util;
mod writebatch;

#[bench]
fn _bench_check_requirement(_: &mut test::Bencher) {
    tikv::util::config::check_max_open_fds(4096).unwrap();
}
