#!/usr/bin/env bash
architecture="aarch64-unknown-linux-gnu"
path="./target/$architecture/debian"
echo "building artifact for $architecture architecture..."
cargo zigbuild --target $architecture --release
cargo deb --target $architecture --no-build
echo "writing debian package to raspberry pi"
pkg=$(ls $path)
scp $path/$pkg pbaba@raspberrypi.local:~/server/wifi-server.deb

