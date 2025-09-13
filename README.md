# xch-keygen

A very simple cli tool for offline generation of BLS keys for use with the chia-blockchain. It derives all the chia things, including hardened addresses. Inspired by https://github.com/scrutinously/key-generator

## Usage

```shell
./xch-keygen [-w <num_words>] [-a <num_addresses>] [-o <offset>] [-s <skip>] [-r [-m <max>]]
```

## Examples

Generate a wallet with with ten addresses derived from indices between 0 and 9.
```shell
./xch-keygen
```

Generate a wallet with with ten addresses derived from every third index starting at 0 (i.e., skipping 2).
```shell
./xch-keygen -s 2
```

Generate a wallet with three addresses derived from random indices between 379 and 1000:

```shell
./xch-keygen -a 3 -o 379 -r -m 1000
```

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