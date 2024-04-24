#!/bin/bash
set -u ; # set -o nounset
set -e ; # set -o errexit
cd wasm-crate ;
ln -sfT "$1/pkg" pkg ; # create a symbolic link to $1/pkg in wasm-crate, that is called pkg
ln -sfT $1 js
wasm-pack build --target web