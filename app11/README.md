
# Structure

Rust test case  in parallel

single thread test case:

``` 
 cargo test -- --test-threads=1
```

## new Lib crate

``` 
cargo new --lib mock-server
```

A cryptographically verifiable code review system for the cargo (Rust) package manager.

``` 
https://github.com/crev-dev/cargo-crev
```

Another crate that you can use is called Cargo Audit (http://mng.bz/VyO5). This
tool will check dependencies against the RustSec Advisory Database

``` 
 cargo install cargo-audit
```

```
cargo audit
```


https://course.rs/about-book.html


解决：切换到nightly版本
``` 
# Install nightly toolchain:
rustup install nightly

# Switch to nightly toolchain
rustup override set nightly
```

其它相关指令
``` 
# Show the defautl toolchain
rustup default

# Set the default toolchain to the latest nightly
rustup default nightly

# Set the default toolchain to the latest stable
rustup default stable

# To use to a specific nightly for a directory:
rustup override set nightly-2014-12-18

# Or a specific stable release:
rustup override set 1.17.0

# To see the active toolchain
rustup show

# To remove the override and use the default toolchain again,
rustup override unset
```