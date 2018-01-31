#!/bin/bash

for number in {1..100}
do
  docker run --net="host" blockchain_peer:latest &
  sleep 2
done