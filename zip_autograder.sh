#! /usr/bin/env bash

cp setup.sh /tmp
echo 'cargo build --manifest-path source/assignment/lib/Cargo.toml' >> /tmp/setup.sh
zip -r autograder.zip /tmp/setup.sh ./run_autograder 'source/assignment/' -x '*/\target/*'
