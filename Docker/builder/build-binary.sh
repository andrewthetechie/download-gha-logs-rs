#!/bin/bash

set -e

echo "Target Platform is: ${TARGETARCH}"
case ${TARGETARCH} in \
         "amd64")  TARGET=x86_64-unknown-linux-musl  ;;
         "arm64")  TARGET=aarch64-unknown-linux-musl ;;
         *) TARGET=x86_64-unknown-linux-musl  ;;
esac

echo "Target is: ${TARGET}"
#rustup target add "${TARGET}"
#cargo build --target "${TARGET}" --release --locked
#cp target/"${TARGET}"/release/download-gha-logs /download-gha-logs
cargo build --release --locked
cp target/release/download-gha-logs /download-gha-logs
chmod +x /download-gha-logs
