{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      flake-utils,
      nixpkgs,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        run-roxy-loader-c-template = pkgs.writeShellApplication {
          name = "run-roxy-loader-c-template";
          runtimeInputs = [
            toolchain
            pkgs.pkgsCross.x86_64-embedded.buildPackages.gcc
            pkgs.gnumake
            pkgs.pkg-config
            pkgs.qemu
            pkgs.cacert
          ];
          text = ''
            make run "$@"
          '';
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            toolchain
            pkgs.pkgsCross.x86_64-embedded.buildPackages.gcc
            pkgs.gnumake
            pkgs.pkg-config
            pkgs.qemu
            pkgs.cacert
          ];
        };

        apps.default = {
          type = "app";
          program = "${run-roxy-loader-c-template}/bin/run-roxy-loader-c-template";
        };
      }
    );
}
