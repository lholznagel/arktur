#! /bin/sh

set -e

# libsodium version
VERSION=1.0.16

wget https://github.com/jedisct1/libsodium/releases/download/$VERSION/libsodium-$VERSION.tar.gz
tar xvfz libsodium-$VERSION.tar.gz
cd libsodium-$VERSION && ./configure --prefix=$HOME/libsodium && make && make install && cd ..