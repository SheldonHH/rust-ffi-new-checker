#!/bin/bash

# 获取当前文件夹名
current_dir=${PWD##*/}

# 设置环境变量
export FFI_CHECKER_TOP_CRATE_NAME=$current_dir
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH

# 清理项目
cargo clean

# 运行 cargo-ffi-checker 并替换 --crate-name 中的值为当前文件夹名
/root/.cargo/bin/cargo-ffi-checker rustc src/main.rs --crate-name=$current_dir
