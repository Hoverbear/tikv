# This Docker image contains minimal binaries for:
#
# * `tikv-server`
# * `tikv-ctl`
# * `tikv-importer`
#
# All TiKV tools are built in **release** mode.
FROM tikv/rust-toolchain as toolchain
ENV ROCKSDB_SYS_SSE 1

WORKDIR ./tikv
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY scripts scripts
COPY Makefile Makefile
COPY etc etc
COPY scripts scripts

# Avoid rebuilding all dependencies every commit
RUN git init
# Ideally here we would do something like `docker build --dependencies-only` to build all the dependencies in a different intermediate step.
# This can result in a time savings of many minutes if the `tikv` crate fails to build at a later step.
#
# You can track this issue here: https://github.com/rust-lang/cargo/issues/2644,
#
# Unfortunately, these aren't an option until then. So instead, we do a hack to keep Docker builds fast.
# If you add a component to TiKV you need to add a line below with the `Cargo.toml` like the others.
#
# We can't:
#  * Use bash to iterate through the `components` directory, because `cmd/` and other components exist outside of this directory.
#    (we'd have to maintain a list anyways, so better to just maintain it here)
#  * Use something like `cargo metadata ... | jq ... | xargs cargo build --package ...` because there is no reliable way to get
#    package specs, and this prevents us from building any dependency which we have multiple versions of.
#  * Docker doesn't support mirror/recursive copy globbing, see https://github.com/moby/moby/issues/39530
#--- Start of dirty hack:
RUN mkdir -p cmd
COPY cmd/Cargo.toml cmd/Cargo.toml
RUN mkdir -p components
RUN mkdir -p components/codec
COPY components/codec/Cargo.toml components/codec/Cargo.toml
RUN mkdir -p components/engine
COPY components/engine/Cargo.toml components/engine/Cargo.toml
RUN mkdir -p components/log_wrappers
COPY components/log_wrappers/Cargo.toml components/log_wrappers/Cargo.toml
RUN mkdir -p components/match_template
COPY components/match_template/Cargo.toml components/match_template/Cargo.toml
RUN mkdir -p components/panic_hook
COPY components/panic_hook/Cargo.toml components/panic_hook/Cargo.toml
RUN mkdir -p components/pd_client
COPY components/pd_client/Cargo.toml components/pd_client/Cargo.toml
RUN mkdir -p components/profiler
COPY components/profiler/Cargo.toml components/profiler/Cargo.toml
RUN mkdir -p components/test_coprocessor
COPY components/test_coprocessor/Cargo.toml components/test_coprocessor/Cargo.toml
RUN mkdir -p components/test_raftstore
COPY components/test_raftstore/Cargo.toml components/test_raftstore/Cargo.toml
RUN mkdir -p components/test_util
COPY components/test_util/Cargo.toml components/test_util/Cargo.toml
RUN mkdir -p components/tidb_query
COPY components/tidb_query/Cargo.toml components/tidb_query/Cargo.toml
RUN mkdir -p components/tidb_query_codegen
COPY components/tidb_query_codegen/Cargo.toml components/tidb_query_codegen/Cargo.toml
RUN mkdir -p components/tidb_query_datatype
COPY components/tidb_query_datatype/Cargo.toml components/tidb_query_datatype/Cargo.toml
RUN mkdir -p components/tikv_alloc
COPY components/tikv_alloc/Cargo.toml components/tikv_alloc/Cargo.toml
RUN mkdir -p components/tikv_util
COPY components/tikv_util/Cargo.toml components/tikv_util/Cargo.toml
RUN mkdir -p components/tipb_helper
COPY components/tipb_helper/Cargo.toml components/tipb_helper/Cargo.toml

RUN bash ./scripts/build-deps-for-docker.sh
#--- End of dirty hack
RUN rm -rf .git
# Now copy over the rest of the code.
COPY Cargo.toml Cargo.toml
COPY cmd cmd
COPY fuzz fuzz
COPY components components
COPY src src
COPY .git .git

RUN make build_dist_release

# Strip debug info to reduce the docker size, may strip later?
# RUN strip --strip-debug /tikv/target/release/tikv-server && \
#     strip --strip-debug /tikv/target/release/tikv-ctl

FROM pingcap/alpine-glibc

COPY --from=toolchain /tikv/target/release/tikv-ctl /tikv-ctl
COPY --from=toolchain /tikv/target/release/tikv-server /tikv-server

EXPOSE 20160

ENTRYPOINT ["/tikv-server"]
