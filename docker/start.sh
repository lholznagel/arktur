#!/bin/bash

for number in {1..50}
do
  docker run -d --net="host" --label peer blockchain_peer:latest
  sleep 2
done