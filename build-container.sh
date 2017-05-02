#!/bin/bash

set -e

cargo build --release
docker build -t rustkata .