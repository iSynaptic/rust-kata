#!/bin/bash

set -e

cargo build --release
docker build --no-cache --force-rm -t rustkata .