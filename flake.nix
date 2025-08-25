{
  description = "telos";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        toolchain =
          with fenix.packages.${system};
          combine [
            default.rustc
            default.cargo
            default.clippy
            default.rustfmt
            complete.miri

            pkgs.rust-analyzer
          ];

        buildInputs = with pkgs; [
          toolchain
          openssl
        ];
        shellPkgs = with pkgs; [ ];
      in
      {
        devShell = pkgs.mkShell {
          inherit buildInputs;
          packages = shellPkgs;
          #CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
          #LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (buildInputs pkgs)}";
        };
      }
    );
}
