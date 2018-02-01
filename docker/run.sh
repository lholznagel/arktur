#!/bin/sh
set -e

exec chpst -unobody /etc/service/blockchain_peer/peer --name "docker_peer"