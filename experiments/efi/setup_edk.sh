#!/bin/bash

EDK2_DIR=~/Downloads/edk2

export WORKSPACE=$(realpath ..)
export PACKAGES_PATH=$WORKSPACE/efi:$EDK2_DIR
source $EDK2_DIR/edksetup.sh

