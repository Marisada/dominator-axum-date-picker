#!/bin/bash

set -e

PWA_PATH="volume/pwa"
PROJECT_PATH=$(pwd)

cd $PWA_PATH

VERSION=$(date '+%Y%m%d-%H%M%S')
echo "const VERSION = '${VERSION}'" > sw.js
cat sw_template.js >> sw.js

echo "update sw.js version to ${VERSION}"
cd ${PROJECT_PATH}

echo -e "\\ndone."
