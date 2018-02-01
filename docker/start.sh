#!/bin/bash

for number in {1..100}
do
  docker run -d --net="host" --label peer blockchain_peer:latest
  #sleep 1
done