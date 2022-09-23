#! /bin/bash

if [ $# -eq 0 ]
        then
                VAR="no message"
else
        VAR="$1"
fi

sh generate_icons.sh

wait

cd werk_server
git add .
git commit -m $VAR
git push