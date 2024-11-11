# Publishing
These are just some general notes for the mainainer of Terminal3d to publish to the Brew tap and other registries.

## Procedure
1) Edit [`Cargo.toml`](./Cargo.toml) and increment to the desired version. Push a commit.
2) Go to [the releases page](https://github.com/liam-ilan/terminal3d/releases), and create a new release, with the same version that's in the [`Cargo.toml`](./Cargo.toml).
3) Run `cargo publish` to publish to [crates.io](https://crates.io/crates/terminal3d).
4) Download the tar from the [latest Github release](https://github.com/liam-ilan/terminal3d/releases), and run `shasum -a 256 <filepath.tar.gz>` to obtain the sha256 hash required for Brew Formulae.
5) Go to the Homebrew tap repo for Terminal3d, [liam-ilan/homebrew-terminal3d](https://github.com/liam-ilan/homebrew-terminal3d), and update:
    - The `url` field with the correct tag.
    - The `sha256` field with the hash from the previous step.
6) Commit and push the changes to the Homebrew tap.