#!/bin/bash

docker build -t rust-project .
docker run -d --name rust-container rust-project
mkdir -p ../../target/release
docker cp rust-container:/app/target/release/libjeb5.so ../../client/src/main/resources/libjeb5.so
docker stop rust-container
docker rm rust-container