#! /bin/bash

git reset --hard
git pull
wait

cd werk_server
git reset --hard
git pull
wait

cd ..
cd werk-web 
git reset --hard
git pull
cd ..