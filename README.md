# xch-keygen

A very simple cli tool for offline generation of BLS keys (read "wallet(s)") for use with the chia-blockchain. It derives all the chia things, including hardened addresses.

## Usage

```shell
./xch-keygen [-w <num_words>] [-a <num_addresses>] [-o <offset>] [-s <skip>] [-r [-m <max>]] [-q] [-e <app_name> [--export-hot]] [--enable-naming] [FILE]
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

### Derive all the things from existing mnemonic:

Prompt for mnemonic:

```shell
./xch-keygen -p
```

Pipe:

```shell
echo "<mnemonic_phrase>" | ./xch-keygen
```

File descriptor stdin:

```shell
./xch-keygen <(echo "<mnemonic_phrase>")
```

Plain file path:

```shell
./xch-keygen <mnemonic_phrase_file>
```

### Exporting:

Export wallet public key to sage, with a fingerprint-derived label:

```
./xch-keygen -e sage --enable-naming
```

Export wallet private key to sage:

```
./xch-keygen -e sage --export-hot
```

Export wallet public key to the chia reference wallet:

```
./xch-keygen -e chia
```

Export wallet public key to the chia reference wallet, but don't output wallet details to stdout:

```
./xch-keygen -q -e chia
```

Export wallet private key to multiple wallet applications:

```
./xch-keygen -e sage chia --export-hot
```


### Scripting
Generate six wallets, with three derived addresses starting from a random offset between 317 and 5002, writing them to the current directory:

```shell
for i in $(shuf -i 317-5002 -n 6 | sort -u); do \
    ./xch-keygen -a 3 -o $i > xch-keygen-$(shuf -n 1 /usr/share/dict/words | awk '{print tolower($0)}').txt; \
done
```

## Building From Source

First, install rust (https://www.rust-lang.org/tools/install).

```shell
git clone https://github.com/Jsewill/xch-keygen.git
cd xch-keygen
cargo build --release
```

## Acknowledgements

Thanks to [scrutinously](https://github.com/scrutinously) and https://github.com/scrutinously/key-generator for the inspiration.