{
  description = "A Nix-flake-based Rust development environment for near-dns-backend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = pkgs.rust-bin.stable.latest.default;
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            rustVersion
            pkgs.rust-analyzer
            pkgs.cargo
            pkgs.openssl
            pkgs.pkg-config
            pkgs.perl
            pkgs.libudev-zero
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.openssl.out}/lib
            export PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig
            exec zsh
          '';
        };
      });
}

