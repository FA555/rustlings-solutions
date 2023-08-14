# Rustlings Solutions

Solutions to rust-lang/rustlings `5.5.1`.

## Usage

Either look up directly in the browser or clone the repo. As if in the original repo, the exercises are in directory `exercises/`.

If you have cloned the repo to local, you can verify the answers using command `rustlings verify` in the directory of the repo.

## But what is in `/src`?

Attached Rust application is a small toy validating whether the file hierarchy (`info.toml` and `exercises/`) can be accepted by rustlings, and the reason for providing it is that we need a `Cargo.toml` file in the root directory {self, containing a(n invalid) dependency to [rustlings original repo](https://github.com/rust-lang/rustlings), which is warned by clippy. }
