#! /bin/sh -ea

CRATE_DIR=$(dirname $0)

cd "${CRATE_DIR}/device-generator"

echo "Generating cores"

cargo run -- -o "${CRATE_DIR}/src/gen"
