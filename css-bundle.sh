#!/bin/bash

set -e

CSS_PATH="volume/pwa/css"

echo "create ${CSS_PATH}/app.min.css"
grass -s compressed picker.scss ${CSS_PATH}/picker.css

echo -e "\\n"
./reload.sh
