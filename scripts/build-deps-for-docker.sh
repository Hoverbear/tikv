#! /bin/bash
set +e

# Support building the dependencies for a docker build in a separate step.
# This script should be run with `pwd` being the `tikv` repo.

# This script exists because of https://github.com/rust-lang/cargo/issues/2644 and 
# `[replace]` doesn't play well with `cargo vendor`.
# This is complicated by https://github.com/rust-lang/cargo/issues/7267

# The general idea is:
# 
# Stub out `/src/lib.rs` for every component and let the dependencies build. Then you can recopy the source and run the build again.
# This build should then be able to only need to build TiKV components.

# TODO: Why do we need to grep out `test` and `profiler`?
components=$(ls -d ./components/* | xargs -n 1 basename | grep -v "test" | grep -v "profiler")
# We must remove the profiler from tidb_query. (TODO: Why do we need to grep out `test` and `profiler`?)
sed -i '/profiler/d' ./components/tidb_query/Cargo.toml

for component in ${components}; do
    mkdir -p ./${component}src/lib.rs
    echo '' > ./${component}/src/lib.rs
done

echo 'fn main() {}' > ./cmd/src/bin/tikv-ctl.rs
echo 'fn main() {}' > ./cmd/src/bin/tikv-server.rs

make build_dist_release

for component in ${components}; do
    rm -rf ./target/release/.fingerprint/${component}-*
done
rm -rf .target/release/.fingerprint/tikv-*