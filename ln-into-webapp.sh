#!/bin/bash
set -u ; # set -o nounset
set -e ; # set -o errexit
[[ -d $1 ]] &&
cd wasm-crate &&
ln -rsT $1 pkg # create a relative, symbolic link to $1 in wasm-crate, that is called pkg 