#! /bin/sh

set -e

# libsodium version
VERSION=1.0.16

curl -L -o libsodium.tar.gz https://download.libsodium.org/libsodium/releases/libsodium-$VERSION.tar.gz
tar xvfz libsodium.tar.gz
cd libsodium-$VERSION && ./configure && make && make install && cd ..
rm -rf libsodium.tar.gz