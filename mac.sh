#! /bin/bash

cd /Users/kyleebner/Development/cnctd/src-tauri
cargo tauri icon

wait

source /Users/kyleebner/.bash_profile

cargo tauri build