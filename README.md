# xch-keygen

A very simple cli tool for offline generation of BLS keys for use with the chia-blockchain. It derives all the chia things, including hardened addresses. Inspired by https://github.com/scrutinously/key-generator

## Usage

```./xch-keygen [-w <num_words>] [-a <num_addresses>] [-o <offset>]```

### Building From Source

First, install rust (https://www.rust-lang.org/tools/install).

```shell
git clone https://github.com/Jsewill/xch-keygen.git
cd xch-keygen
cargo build --release
```