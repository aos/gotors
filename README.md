## gotors (`goto` in Rust)

This is a re-implementation of my [`goto`](https://github.com/aos/goto) CLI in Rust.

### Changes from original

- Subcommands instead of options for listing, adding, and init
- Attempts to place the `gotorc` file in the `$XDG_CONFIG_HOME` first before
  `$HOME`.
- A `flake.nix` to pull the package down into your own Nix configuration.
