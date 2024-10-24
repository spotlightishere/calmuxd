{
  description = "calmuxd service and NixOS module";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, flake-utils, nixpkgs }:
    # There's several things going on here:
    # We'll begin with per-Nix system components.
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        # We need to link against a few additional packages.
        darwinPkgs = with pkgs; (lib.optional stdenv.hostPlatform.isDarwin (with darwin.apple_sdk.frameworks; [
          libiconv
          CoreFoundation
          Security
          SystemConfiguration
        ]));
      in
      rec {
        # First, a simple shell to permit Rust development.
        devShells.default = with pkgs; mkShell {
          nativeBuildInputs = [ pkg-config ];

          buildInputs = [
            cargo
            clippy
            rustc

            openssl
          ] ++ darwinPkgs;

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };

        # Next, the raw `calmuxd` package itself.
        packages = {
          calmuxd = with pkgs; rustPlatform.buildRustPackage {
            pname = "calmuxd";
            version = "0.1.0";

            nativeBuildInputs = [ pkg-config ];
            buildInputs = [ openssl ] ++ darwinPkgs;

            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            meta = with lib; {
              description = "Simple calendar feed muxing agent";
              homepage = "https://github.com/spotlightishere/calmuxd";
              license = licenses.mit;
              maintainers = with maintainers; [ spotlightishere ];
            };
          };
          default = self.packages.${system}.calmuxd;
        };

        # Our preferred Nix formatter.
        formatter = pkgs.nixpkgs-fmt;
      }))
    //
    {
      # Provide a NixOS module defining our service and config.
      nixosModules.default = import ./module.nix { inherit self; };
    };
}
