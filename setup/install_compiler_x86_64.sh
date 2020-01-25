#!/bin/bash

requireArch="x86_64"
currentArch=$(arch)

if [ "${currentArch}" != "${requireArch}" ]; then
  echo "Not an ${requireArch} architecture."
  exit 1
fi

apt-get install -y \
  gcc-8 \
  gcc-8-arm-linux-gnueabihf \
  gcc-8-aarch64-linux-gnu \
  gcc-8-i686-linux-gnu \
  mingw-w64
