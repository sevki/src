ARG VARIANT "bookworm-20240701"

# FROM debian:${VARIANT}
FROM debian:bookworm-20240701

RUN apt-get update && apt-get install -y \
    sudo \
    libssl-dev \
    lsb-release \
    musl-dev \
    make \
    cmake \
    git \
    qemu-kvm \
    perl \
    gzip \
    ca-certificates \
    clang \
    cloud-image-utils \
    curl \
    dpkg-dev \
    expect \
    g++ \
    gcc \
    git \
    jq \
    libavcodec-dev \
    libc++abi1-14 \
    libavutil-dev \
    libcap-dev \
    libclang-dev \
    libdbus-1-dev \
    libdrm-dev \
    libepoxy-dev \
    libglib2.0-dev \
    libguestfs-tools \
    libslirp-dev \
    libssl-dev \
    libswscale-dev \
    libva-dev \
    libunwind-dev \
    libunwind-14 \
    libwayland-dev \
    libxext-dev \
    lld \
    make \
    meson \
    mypy \
    nasm \
    npm \
    ncat \
    ninja-build \
    openssh-client \
    pipx \
    pkg-config \
    protobuf-compiler \
    python3 \
    python3-argh \
    python3-pip \
    python3-rich \
    qemu-system-x86 \
    rsync \
    screen \
    strace \
    tmux \
    wayland-protocols \
    wget \
    && rm -rf /var/lib/apt/lists/*


# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable -y
ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_HOME=/root/.cargo

# install wasm32-unknown-unknown
RUN rustup target add wasm32-unknown-unknown

# update rust
RUN rustup update

RUN curl https://raw.githubusercontent.com/SeaQL/FireDBG.for.Rust/main/install.sh -sSf | sh

WORKDIR /workspaces