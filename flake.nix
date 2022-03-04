{
  description = "blog submission for Xe's blog challenge 2022 -- used to learn Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustEnv = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);

      in
      rec {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              rustEnv
              openssl
              pkgconfig
              cargo-edit
              cargo-watch
            ];
        };
      });
}
