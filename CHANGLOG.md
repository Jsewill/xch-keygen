# Changelog

## 0.0.3 - 2025-09-14

### Added

- Derivation from existing mnemonic phrase by:
 - File path: `./xch-keygen [...] [FILE]`.
 - Prompt: `-p --phrase`.
 - stdin/pipe.
 - File descriptor style redirection: `./xch-keygen <(echo "<mnemonic_seed_phrase>")`.

## 0.0.2 - 2025-09-12

### Added

- Address index stepping via `-s --skip`.
- Address index randomness via `-r --random`.
- Address index randomness max range via `-m --max`.