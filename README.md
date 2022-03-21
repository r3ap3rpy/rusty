### Welcome

This repository holds source materials for my [Rust](https://www.rust-lang.org) programming class.

The source files are guaranteed to compile on a [Fedora](https://getfedora.org/hu/) workstation system after installing the following dependencies.

``` bash
sudo su -
dnf upgrade --refresh -y
dnf install curl dnf-plugins-core install cmake gcc clang make -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
You can leave everything on default.

Once the installation completes you can issue the following command to verify.

``` bash
rustc --version
```

The source files in this repository will make sense following my course!