#! /bin/sh -ea

CRATE_DIR=$(realpath $(dirname $0))

echo "create dir: '${CRATE_DIR}'"

cd "${CRATE_DIR}/device_generator"

echo "Generating cores"

cargo run -- -o "${CRATE_DIR}/src/gen"
