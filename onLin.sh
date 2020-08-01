#!/bin/bash

docker run --rm -it -v ~/works/github/cc9rust/:/home/cc9 -w /home/cc9 rust $@
