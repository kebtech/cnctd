#! /bin/bash

if [ $# -eq 0 ]
        then
                VAR="no message"
else
        VAR="$1"
fi

cd werk-web
git add .
git commit -m $VAR
git push

cd ..
git add .
git commit -m $VAR
git push

sh generate_icons.sh

wait

cd werk_server
git add .
git commit -m $VAR
git push

wait

heroku logs -a cnctd --tail