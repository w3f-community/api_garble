################################################################################
# cargo install --path . --root target/install/
# mkdir ./target/install/lib/
# find target/release/build/ -type f -name "*.so*" -exec cp  {} ./target/install/lib/ \;
# docker build -f Dockerfile_M1.dockerfile -t api_garble:dev .
# docker run -it --name api_garble --rm -p 3000:3000 --env RUST_LOG="warn,info,debug" api_garble:dev /usr/local/bin/api_garble --ipfs-server-multiaddr /ip4/172.17.0.1/tcp/5001

FROM rust:1.59 as builder

ENV APP_NAME api_garble

WORKDIR /usr/src/app

# TODO use this
# prereq: install CMake, Ninja, etc
# COPY . .
# RUN cargo install --path .

# directly copy the result of "cargo install" in the host local folder
COPY target/install/bin/$APP_NAME /usr/local/cargo/bin/$APP_NAME
# MUST also get all the shared libs
ADD target/install/lib /usr/local/lib/$APP_NAME/

################################################################################

FROM ubuntu:20.04

EXPOSE 3000

ENV APP_NAME api_garble
ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib

# TODO remove patchelf once proper PROD container
# RUN apt-get update && apt-get install -y libboost-filesystem-dev libpng-dev libreadline-dev libtcl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/lib/$APP_NAME /usr/local/lib/
COPY --from=builder /usr/local/cargo/bin/$APP_NAME /usr/local/bin/$APP_NAME
# that is really ugly; we MUST fix some lib SONAME/path
# TODO patchelf?
RUN mv /usr/local/lib/libglog.so.0.6.0 /usr/local/lib/libglog.so.1

CMD ["sh", "-c", "$APP_NAME"]