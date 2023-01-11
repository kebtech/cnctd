#! /bin/bash

#increment version
npm version patch --no-git-tag-version

# wait

# version=$(grep version package.json | sed 's/.*"version": "\(.*\)".*/\1/')

# echo "${version}"