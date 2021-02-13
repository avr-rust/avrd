#! /bin/sh -ea

CRATE_DIR=$(realpath $(dirname $0))

cd "${CRATE_DIR}/device_generator"

echo "Generating cores"

cargo run -- -o "${CRATE_DIR}/src/gen" $@
