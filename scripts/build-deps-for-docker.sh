#! /bin/bash
set +e

# Support building the dependencies for a docker build in a separate step.
# This script should be run with `pwd` being the `tikv` repo.

# The general idea is: 
# Stub out `/src/lib.rs` for every component and let the dependencies build. Then you can recopy the source and run the build again.
# This build should then be able to only need to build TiKV components.
components=$(ls -d ./components/* | xargs -n 1 basename | grep -v "test")
sed -i '/fuzz/d' ./Cargo.toml
sed -i '/test\_/d' ./Cargo.toml

rm -rf ./src
mkdir -p ./src
echo '' > ./src/lib.rs
for component in ${components}; do
    echo ${component}
    mkdir -p ./components/${component}/src/
    echo '' > ./components/${component}/src/lib.rs
done
mkdir -p ./components/profiler/examples/
echo 'fn main() {}' > ./components/profiler/examples/prime.rs

# TODO: Unclear why not in `components/`
rm -rf ./cmd/src
mkdir -p ./cmd/src/bin
echo 'fn main() {}' > ./cmd/src/bin/tikv-ctl.rs
echo 'fn main() {}' > ./cmd/src/bin/tikv-server.rs
echo '' > ./cmd/src/lib.rs

make build_dist_release

for component in ${components}; do
    rm -rf ./target/release/.fingerprint/${component}-*
done
rm -rf .target/release/.fingerprint/tikv-*