#!/bin/bash

# import log library
source ../../build/b-log.sh

# enable all level log
LOG_LEVEL_ALL

INFO "Build XPDF Env!"
cp -rf ../../goals/xpdf-3.02.tar.gz .
tar xvf xpdf-3.02.tar.gz
rm xpdf-3.02.tar.gz
mv xpdf-3.02 xpdf
INFO "Let's start fuzzing!"