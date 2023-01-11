#! /bin/bash

cd /Users/kyleebner/Development/cnctd/src-tauri
cargo tauri icon

wait

source /Users/kyleebner/.bash_profile

version=$(npm version patch --no-git-tag-version)

patch=$(${version##*.})
echo ${patch}

cargo tauri build