{
  description = "blog submission for Xe's blog challenge 2022 -- used to learn Rust";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustEnv = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
        naersk-lib = naersk.lib."${system}";

        start-server = pkgs.writeShellScriptBin "start-server" ''
          ${pkgs.cargo-watch}/bin/cargo-watch -x 'run'
        '';

      in
      rec {
        packages.xeBlogChallenge2022 = naersk-lib.buildPackage {
          pname = "xeBlogChallenge2022";
          root = ./.;
          buildInputs = [
            pkgs.sqlite
            pkgs.openssl
          ];
          # doDoc = true;
          fixupPhase = ''
            cp -r ./* $out
          '';
        };
        defaultPackage = packages.xeBlogChallenge2022;

        packages.docker =
          let
            blog = self.defaultPackage.${system};
          in
          pkgs.dockerTools.buildLayeredImage {
            name = blog.pname;
            tag = "${self.lastModifiedDate}-${self.shortRev or "dirty"}";
            contents = [ blog ];

            config = {
              Cmd = [ "/bin/ajaxbits" ];
              WorkingDir = "/";
            };
          };


        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              rustEnv
              openssl
              pkgconfig
              cargo-edit
              cargo-watch
              sqlite

              start-server
            ];
        };
      });
}
