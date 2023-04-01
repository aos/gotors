{
  description = "Fast jump between directories";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, utils, rust-overlay, naersk, ... }:
    {
      overlays.default = final: prev: {
        gotors = self.packages.${prev.stdenv.hostPlatform.system}.gotors;
      };
    } //
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
            ${name} = naersk.lib.${system}.buildPackage {
              pname = name;
              root = ./.;
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
      );
}
