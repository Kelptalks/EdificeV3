#!/bin/bash
set -e

cargo build --release

mkdir -p releases
cp target/release/EdificeV3 releases/
cp -r Assets releases/
cp -r Lua_scripts releases/

echo "Release built: releases/"
