# 走了
```bash
# ubuntu:22.04
apt update -y
apt install -y git curl vim sudo build-essential zlib1g-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
PATH="/root/.cargo/bin:${PATH}"
source $HOME/.cargo/env
```


更新 Rust 编译器版本：
```bash
rustup update stable
rustup default stable
# replace rustup-toolchain as stable
```
Install LLVM 13
```bash
sudo apt update -y
sudo apt install llvm-13 llvm-13-dev clang-13 -y

ls /usr/lib/llvm-13
export LLVM_SYS_130_PREFIX=/usr/lib/llvm-13
export PATH=/usr/lib/llvm-13/bin:$PATH

```

失败之后，switch to nightly toolchain
```bash
rustup install nightly
rustup default nightly
```

之后Install required components:
```bash
rustup component add rust-src
rustup component add rustc-dev
rustup component add llvm-tools-preview
```

# FFIChecker: A Static Analysis Tool For Detecting Memory Management Bugs Between Rust and C/C++

[![build](https://github.com/lizhuohua/rust-ffi-checker/actions/workflows/build.yml/badge.svg)](https://github.com/lizhuohua/rust-ffi-checker/actions/workflows/build.yml)

This tool generates and analyzes LLVM bitcode to detect potential bugs caused by incorrect use of Rust FFI.

Information about bugs detected by this tool are listed in [Trophy Case](trophy-case/README.md).

## Requirements

* Rust nightly, as specified in [rust-toolchain](rust-toolchain).
* `rustc-dev` and `llvm-tools-preview`:

    ```sh
    $ rustup component add rustc-dev llvm-tools-preview
    ```

* `LLVM 13`:

    ```sh
    # Some required libraries are included in 'libclang-common-13-dev'
    $ sudo apt-get install llvm-13-dev libclang-common-13-dev
    ```

## Build

1. Clone the repository

    ```sh
    $ git clone https://github.com/lizhuohua/rust-ffi-checker.git
    
    $ cd rust-ffi-checker
    ```

2. Build & Install

    ```sh
    # You can build and install the cargo subcommand:
    $ cargo install --path .
    
    # Or, you can only build the checker itself:
    $ cargo build
    ```

## Example

The following is a contrived example which contains a use-after-free bug. For more examples, please see [examples](examples) and [trophy-case](trophy-case).

```rust
use libc::{c_void, free};

fn main() {
    let mut n = Box::new(1);
    unsafe {
        free(&mut *n as *const _ as *mut c_void);
    }

    *n = 2;
}
```

It compiles but will crash at runtime. Our checker can detect it at compile time.

## Usage

Before using this tool, make sure your Rust project compiles without any errors or warnings.

```sh
# If you have installed the cargo subcommand:
$ cargo clean; cargo ffi-checker

# Or, you can directly run the checker binary
$ cargo clean; path/to/cargo-ffi-checker ffi-checker
```

You can also set the threshold of warnings to filter out false positives.
```sh
# Only output warnings with at least medium severity
# Available options: "high", "mid", and "low"
$ cargo clean; cargo ffi-checker -- --precision_filter mid
```

## Debug

Set `RUST_LOG` environment variable to enable logging:

```sh
# Enable all logging
$ export RUST_LOG=rust_ffi_checker

# Can also set logging level
$ export RUST_LOG=rust_ffi_checker=debug
```



## 解决bug
```bash
root@3d50d2cc2a6a:~/cxx_memory_relation# cargo clean
/root/.cargo/bin/cargo-ffi-checker ffi-checker
     Removed 0 files
/root/.cargo/bin/cargo-ffi-checker: error while loading shared libraries: librustc_driver-339e2dcdcdc9cf07.so: cannot open shared object file: No such file or directory
```

### 设置 LD_LIBRARY_PATH：
```bash
export FFI_CHECKER_TOP_CRATE_NAME=cxx_memory
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
```
#### 确保在项目根目录中存在 rust-toolchain 文件，内容如下：
```toml
[toolchain]
channel = "nightly"
components = ["rustc-dev", "llvm-tools-preview"]
```

#### 
```bash
export FFI_CHECKER_TOP_CRATE_NAME=cxx_memory_relation

# 清理项目
cargo clean

# 设置 LD_LIBRARY_PATH
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH

# 运行 cargo-ffi-checker
/root/.cargo/bin/cargo-ffi-checker ffi-checker -- --precision_filter mid
```


For more settings, please see the documents of [env_logger](https://crates.io/crates/env_logger).

## Troubleshooting

For macOS, you may encounter `dyld: Library not loaded` error, try setting:

```sh
$ export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
```

## License

See [LICENSE](LICENSE)
# rust-ffi-checker
# rust-ffi-checker
# rust-ffi-new-checker
