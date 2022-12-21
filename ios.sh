#! /bin/bash

cd werk-web
cordova-res ios --skip-config --copy
npx cap sync

wait

npx cap open ios

cd ..
