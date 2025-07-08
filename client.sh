#!/bin/bash

set -e

NAME="client"
PWA_PATH="volume/pwa"
WASM_PATH="frontend"
PROJECT_PATH=$(pwd)

cd $PWA_PATH

if [ -e "${NAME}_bg.wasm" ]; then
    rm ${NAME}_bg.wasm
fi
if [ -e "${NAME}.js" ]; then
    rm ${NAME}.js
fi
if [ -e "sw.js" ]; then
    rm sw.js
fi

cd ${PROJECT_PATH}/${WASM_PATH}

if ($( wasm-pack build --target web --out-name $NAME --out-dir wasm-pack/ --$MODE )) ; then
    echo -e "\\n"
    mv wasm-pack/${NAME}_bg.wasm ${PROJECT_PATH}/${PWA_PATH}/
    mv wasm-pack/${NAME}.js ${PROJECT_PATH}/${PWA_PATH}/
else
    cd ${PROJECT_PATH}
    exit 1
fi

cd ${PROJECT_PATH}/${PWA_PATH}

if [ -e "${NAME}_bg.wasm" ]; then
    echo "build ${PROJECT_PATH}/${PWA_PATH}/${NAME}_bg.wasm successfully"
fi
if [ -e "${NAME}.js" ]; then
    echo "build ${PROJECT_PATH}/${PWA_PATH}/${NAME}.js successfully"
fi

VERSION=$(date '+%Y%m%d-%H%M%S')
echo "const VERSION = '${VERSION}'" > sw.js
cat sw_template.js >> sw.js

echo "update sw.js version to ${VERSION}"

cd ${PROJECT_PATH}