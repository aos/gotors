{
  description = "Fast jump between directories";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-21.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, rust-overlay, ... }:
    utils.lib.eachDefaultSystem
      (system:
        let
          name = "gotors";
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
              (self: super: {
                rustc = self.rust-bin.stable.latest.default;
                cargo = self.rust-bin.stable.latest.default;
              })
            ];
          };
        in
        rec {
          # nix build
          packages = {
            ${name} = pkgs.rustPlatform.buildRustPackage rec {
              pname = name;
              version = "1.0.0";
              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              meta.mainProgram = "gotors";
            };
            default = packages.${name};
          };

          # nix develop
          devShells.default = pkgs.mkShell {
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            inputsFrom = builtins.attrValues self.packages.${system};
            buildInputs = with pkgs; [
              cargo
              rust-analyzer
              rustfmt
            ];
          };
        }
      ) //
      {
        overlays.default = final: prev: {
          gotors = self.packages.${prev.stdenv.hostPlatform.system}.gotors;
        };
      };
}
