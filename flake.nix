{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        # Fix duplicate instances of these inputs by pointing them to my inputs
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];
      # Read from `rust-toolchain.toml` instead of adding `rust-bin.nightly.latest.default` to devShell `buildInputs`
      rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      naerskLib = pkgs.callPackage naersk {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };

      pkgs = import nixpkgs {
        inherit system overlays;
      };


      # Libraries that are mostly needed for tauri to work
      libraries = with pkgs; [
      ];

      packages = with pkgs; [
      ];

      # Inputs needed at compile-time
      nativeBuildInputs = with pkgs; [ rustToolchain ];
      # Inputs needed at runtime
      buildInputs = with pkgs; [ ] ++ packages ++ libraries;
    in
    {
      packages.default = naerskLib.buildPackage {
         src = ./.;
      };

      devShells = {
        default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}
          '';
        };
      };
    });
}
