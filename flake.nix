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
        srcNoTarget = dir:
          builtins.filterSource
            (path: type: type != "directory" || builtins.baseNameOf path != "target")
            dir;

        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustEnv = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
        naersk-lib = naersk.lib."${system}";

        start-server = pkgs.writeShellScriptBin "start-server" ''
          ${pkgs.cargo-watch}/bin/cargo-watch -x 'run'
        '';

        src = srcNoTarget ./.;
        blog = naersk-lib.buildPackage {
          inherit src;
          name = "xeBlogChallenge2022";
          buildInputs = [
            pkgs.sqlite
            pkgs.openssl
          ];
          remapPathPrefix = true;
          DATABASE_URL = "sqlite://posts.db";
        };

      in
      rec {
        packages.xeBlogChallenge2022 = pkgs.stdenv.mkDerivation {
          inherit (blog) name;
          inherit src;
          phases = "installPhase";

          installPhase = ''
            mkdir -p $out $out/bin

            cp -rf $src/static $out/static
            cp -rf $src/posts.db $out/posts.db
            cp -rf $src/templates $out/templates

            cp -rf ${blog}/bin/ajaxbits $out/bin/ajaxbits
          '';
        };
        defaultPackage = packages.xeBlogChallenge2022;

        packages.docker =
          let
            site = self.defaultPackage.${system};
          in
          pkgs.dockerTools.buildLayeredImage {
            name = site.name;
            tag = "develop";
            # tag = "${self.lastModifiedDate}-${self.shortRev or "dirty"}";
            contents = [ site ];

            config = {
              Cmd = [
                "/bin/ajaxbits"
              ];
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

              flyctl
            ];
        };
      });
}
