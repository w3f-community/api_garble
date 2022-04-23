################################################################################

# podman build -f Dockerfile -t api_garble:dev --volume ~/.cargo:/root/.cargo:rw --volume $(pwd)/target/release:/usr/src/app/target/release:rw .
# NOTE: it CAN work with Docker but it less than ideal b/c it can not reuse the host's cache
# NOTE: when dev/test: if you get "ninja: error: loading 'build.ninja': No such file or directory"
# -> FIX: find target/release/ -type d -name "*-wrapper-*" -exec rm -rf {} \;
# b/c docker build has no support for volume contrary to podman/buildah
# docker run -it --name api_garble --rm -p 3001:3000 --env RUST_LOG="warn,info,debug" api_garble:dev /usr/local/bin/api_garble --ipfs-server-multiaddr /ip4/172.17.0.1/tcp/5001

FROM rust:1.59 as builder

WORKDIR /usr/src/app

# prereq: install CMake
ENV PATH=$PATH:/opt/cmake/bin/
RUN wget https://github.com/Kitware/CMake/releases/download/v3.22.3/cmake-3.22.3-linux-x86_64.sh && \
    chmod +x cmake-3.22.3-linux-x86_64.sh && \
    mkdir /opt/cmake/ && \
    ./cmake-3.22.3-linux-x86_64.sh --skip-license --prefix=/opt/cmake/ && \
    rm cmake-*.sh && \
    cmake -version

# prereq: install Ninja (ninja-build)
RUN wget https://github.com/ninja-build/ninja/releases/download/v1.10.2/ninja-linux.zip && \
    unzip ninja-linux.zip -d /usr/local/bin/ && \
    rm ninja-linux.zip && \
    ninja --version

# "error: 'rustfmt' is not installed for the toolchain '1.59.0-x86_64-unknown-linux-gnu'"
RUN rustup component add rustfmt

RUN apt-get update && apt-get install -y \
    libboost-filesystem-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo install --path .

# MUST also get all the shared libs; using the CMake generated list of libs
# cf https://github.com/Interstellar-Network/cmake/blob/main/export_libs.cmake
# It SHOULD be empty for "lib_garble" but we might as well handle it just in case.
# NOTE: if it fails with cp: will not overwrite just-created '/usr/local/lib/liblibyosys.so' with '/usr/src/app/target/release/build/lib-circuits-wrapper-a097322ac7999802/out/build/_deps/yosys_fetch-build/liblibyosys.so'
# It probably means you are caching the container target/ by using a volume and there are multiple build dir
# CHECK: find target/release/build/ -type d -name "*lib-garble-wrapper*"
# If yes: DELETE the dup
RUN cat $(find target/release/ -type f -name cmake_generated_libs) | tr " " "\n" |  grep "/usr/src/app/target/release/.*.so" > list_local_shared_libs && \
    xargs --arg-file=list_local_shared_libs cp --target-directory=/usr/local/lib/ && \
    rm list_local_shared_libs \
    || echo "no shared libs to copy" && touch /usr/local/lib/no_shared_lib_to_copy

################################################################################

FROM ubuntu:20.04

EXPOSE 3000

ENV APP_NAME api_garble

ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib

RUN apt-get update && apt-get install -y \
    libfreetype6 \
    && rm -rf /var/lib/apt/lists/*

# NOTE if "no shared libs to copy" above; we  MUST add a random file else COPY fails with:
# "copier: stat: ["/usr/local/lib/*.so"]: no such file or directory"
# cf https://stackoverflow.com/questions/31528384/conditional-copy-add-in-dockerfile
COPY --from=builder /usr/local/lib/no_shared_lib_to_copy /usr/local/lib/*.so /usr/local/lib/
COPY --from=builder /usr/local/cargo/bin/$APP_NAME /usr/local/bin/$APP_NAME
# TODO use CMake install and DO NOT hardcode a path
COPY --from=builder /usr/src/app/lib_garble_wrapper/deps/lib_garble/data /usr/src/app/lib_garble_wrapper/deps/lib_garble/data/

CMD ["sh", "-c", "$APP_NAME"]