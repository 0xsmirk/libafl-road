#!/bin/bash

# import log library
source ../../build/b-log.sh

# enable all level log
LOG_LEVEL_ALL

# copy openssl source code to here
INFO "Copy Openssl Source Code To Here!"
tar -zxvf ../../goals/openssl-1.0.1f.tar.gz -C .
mkdir crash_output
INFO "Let's start fuzzing"