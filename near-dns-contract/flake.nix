{
  description = "Near-DNS smart contract in rust";

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

        rustVersion = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        cargo-near = pkgs.stdenv.mkDerivation rec {
          pname = "cargo-near";
          version = "0.13.4";
          src = pkgs.fetchurl {
            url = "https://github.com/near/cargo-near/releases/download/cargo-near-v${version}/cargo-near-x86_64-unknown-linux-gnu.tar.gz";
            sha256 = "sha256-M9HKpg+Lo3VWnL3XQ9++iNu7qMezZ6aVpMlPMhkKY6A=";
          };
          nativeBuildInputs = [ pkgs.autoPatchelfHook ];
          buildInputs = [
            pkgs.stdenv.cc.cc.lib
            pkgs.openssl
            pkgs.zlib
            pkgs.glibc
            pkgs.libudev-zero
          ];
          installPhase = ''
            mkdir -p $out/bin
            tar -xzf $src -C $out/
            mv $out/cargo-near-x86_64-unknown-linux-gnu/cargo-near $out/bin/cargo-near
            chmod +x $out/bin/cargo-near
            runHook postInstall
          '';
        };
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
            cargo-near
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.openssl.out}/lib
            export PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig
            exec zsh
          '';
        };
      });
}

