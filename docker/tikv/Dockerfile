# This Docker image contains minimal binaries for:
#
# * `tikv-server`
# * `tikv-ctl`
#
# All TiKV tools are built in **release** mode.
FROM tikv/rust-toolchain as toolchain
ENV ROCKSDB_SYS_SSE 1

WORKDIR ./tikv

COPY . /tikv/
RUN cargo fetch
RUN make build_dist_release

FROM pingcap/alpine-glibc

COPY --from=toolchain /tikv/target/release/tikv-ctl /tikv-ctl
COPY --from=toolchain /tikv/target/release/tikv-server /tikv-server

EXPOSE 20160

ENTRYPOINT ["/tikv-server"]
