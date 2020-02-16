#!/bin/bash

requireArch="x86_64"
currentArch=$(arch)

if [ "${currentArch}" != "${requireArch}" ]; then
  echo "Not an ${requireArch} architecture."
  exit 1
fi

archiveName=android-ndk-r20b-linux-x86_64.zip

wget https://dl.google.com/android/repository/android-ndk-r20b-linux-x86_64.zip?hl=ja -O $archiveName
unzip $archiveName
rm -f $archiveName

mv ./android-ndk-r20b ../libs