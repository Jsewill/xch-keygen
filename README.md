# xch-keygen

A very simple cli tool for offline generation of BLS keys for use with the chia-blockchain. It derives all the chia things, including hardened addresses.

## Usage

```./xch-keygen [-w <words>]```

### Building From Source

First, install rust (https://www.rust-lang.org/tools/install).

```shell
git clone https://github.com/jsewill/xch-keygen.git
cd xch-keygen
cargo build --release
```