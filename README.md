# xch-keygen

A very simple cli tool for offline generation of BLS keys for use with the chia-blockchain. It derives all the chia things, including hardened addresses. Inspired by https://github.com/scrutinously/key-generator

## Usage

```./xch-keygen [-w <num_words>] [-a <num_addresses>] [-o <offset>]```

## Examples

Generate six wallets, with three derived addresses starting from a random offset between 317 and 5002, writing them to the current directory:

```shell
for i in $(shuf -i 317-5002 -n 6 | sort -u); do \
    ./xch-keygen -a 3 -o $i > xch-keygen-$(shuf -n 1 /usr/share/dict/words | awk '{print tolower($0)}').txt; \
done
```

### Building From Source

First, install rust (https://www.rust-lang.org/tools/install).

```shell
git clone https://github.com/Jsewill/xch-keygen.git
cd xch-keygen
cargo build --release
```