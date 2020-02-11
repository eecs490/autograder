#! /usr/bin/env bash

zip -r autograder.zip ./setup.sh ./run_autograder 'source/assignment/' -x '*/\target/*'
