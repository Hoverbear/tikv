# This Docker image contains a minimal build environment for TiKV
#
# It contains all the tools necessary to reproduce official production builds of TiKV

# We need to use CentOS 7 because many of our users choose this as their deploy machine.
# Since the glibc it uses (2.17) is from 2012 (https://sourceware.org/glibc/wiki/Glibc%20Timeline)
# it is our lowest common denominator in terms of distro support.
FROM centos:7.6.1810

# We require epel packages, so enable the fedora EPEL repo
RUN yum install -y epel-release

# Install the system dependencies
# Attempt to clean and rebuild the cache to avoid 404s
RUN yum clean all \
	&& yum makecache \
	&& yum update -y \
	&& yum install -y tar wget git which file unzip python-pip openssl-devel \
		make cmake3 gcc gcc-c++ libstdc++-static pkg-config psmisc gdb \
		libdwarf-devel elfutils-libelf-devel elfutils-devel binutils-devel golang \
	&& yum clean all

# CentOS gives cmake 3 a weird binary name, so we link it to something more normal
# This is required by many build scripts, including ours.
RUN ln -s /usr/bin/cmake3 /usr/bin/cmake

# Install Rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path --default-toolchain none -y
ENV PATH /root/.cargo/bin/:$PATH
ENV LIBRARY_PATH /usr/local/lib:$LIBRARY_PATH
ENV LD_LIBRARY_PATH /usr/local/lib:$LD_LIBRARY_PATH

# Make the expected Rust toolchain default
COPY rust-toolchain /rust-toolchain
RUN rustup default $(tail -n 1 rust-toolchain) \
	&& rustup component add rustfmt --toolchain $(tail -n 1 rust-toolchain) \
	&& rustup component add clippy --toolchain $(tail -n 1 rust-toolchain)

ENTRYPOINT ["/bin/bash"]
