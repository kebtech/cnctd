#! /bin/bash

cp manifest.json werk_server/static
cd werk-web
npx generate-icons --manifest ../werk_server/static/manifest.json public/icon.svg
cordova-res android --skip-config --copy
cordova-res ios --skip-config --copy