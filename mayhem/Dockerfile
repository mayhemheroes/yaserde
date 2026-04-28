# Build Stage
FROM rust:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN rustup toolchain install nightly &&     rustup target add --toolchain nightly x86_64-unknown-linux-gnu &&     rustup component add --toolchain nightly rust-src &&     cargo +nightly install cargo-fuzz

RUN echo building instrumented harnesses &&     bash -c "pushd yaserde/fuzz && cargo +nightly fuzz build && popd" &&     mv yaserde/fuzz/target/x86_64-unknown-linux-gnu/release/deserialize /deserialize &&     echo done

RUN echo building non-instrumented harnesses &&     export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" &&     bash -c "pushd yaserde/fuzz && cargo +nightly build --release && popd" &&     mv yaserde/fuzz/target/release/deserialize /deserialize_no_inst &&     echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /deserialize /deserialize_no_inst /
