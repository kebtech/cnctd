#! /bin/bash

cp manifest.json werk_server/static
cd werk-web
npx generate-icons --manifest ../werk_server/static/manifest.json public/cnctd_globe_logo3.svg
