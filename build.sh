#!/bin/sh

set -ex
wasm-pack build --target web --out-dir docs
