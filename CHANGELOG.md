# Changelog

## [0.0.5] - 2025-09-30

### Added

- Added public keys used for each address to output.
- Updated output formmatting.

## [0.0.4] - 2025-09-26

### Added

- Quiet: `-q --quiet`
- Export to wallet application: `-e --export`
  - Export private or public keys to Sage wallet and/or the Chia reference wallet via their respective RPC interfaces.
- Enable fingerprint derived wallet naming: `--enable-naming`
- Added Master Private Key and Farmer Private Key to output.

### Fixed

- Corrected changelog path.

## [0.0.3] - 2025-09-14

### Added

- Derivation from existing mnemonic phrase by:
  - File path: `./xch-keygen [...] [FILE]`.
  - Prompt: `-p --phrase`.
  - stdin/pipe.
  - File descriptor style redirection: `./xch-keygen <(echo "<mnemonic_seed_phrase>")`.

## [0.0.2] - 2025-09-12

### Added

- Address index stepping via `-s --skip`.
- Address index randomness via `-r --random`.
- Address index randomness max range via `-m --max`.