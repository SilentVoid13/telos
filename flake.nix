{
  description = "telos";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    naersk,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default];
        };

        toolchain = with fenix.packages.${system};
          combine [
            default.rustc
            default.cargo
            default.clippy
            default.rustfmt
          ];

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };

        buildInputs = with pkgs; [openssl];
        shellPkgs = with pkgs; [];
      in rec {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          inherit buildInputs;
        };

        devShell = pkgs.mkShell {
          inputsFrom = [defaultPackage];
          packages = shellPkgs;
          #CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
          #LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (buildInputs pkgs)}";
        };
      }
    );
}
