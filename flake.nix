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
      in
      rec {
        # First, a simple shell to permit Rust development.
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            clippy
            rustc
          ] ++ lib.optional stdenv.hostPlatform.isDarwin libiconv;

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };

        # Next, the raw `calmuxd` package itself.
        packages = {
          calmuxd = pkgs.rustPlatform.buildRustPackage {
            pname = "calmuxd";
            version = "0.1.0";

            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            meta = with pkgs.lib; {
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
      }));
}