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
        config = {
          allowUnfree = true;
        };
      };


      # These are mostly needed for raylib to build
      packages = with pkgs; [
        # [Raylib]
        cmake
      ] ++ pkgs.lib.lists.optionals stdenv.isLinux [
        # [Raylib]
        clang
        libclang
        libcxx # C++ std library
        libGL

        xorg.libX11
        xorg.libXrandr
        xorg.libXinerama
        xorg.libXcursor
        xorg.libXi
        xorg.libXi
        libatomic_ops
        mesa
        alsa-lib

        glibc

        # [Wayland]
        wayland
        wayland-protocols
        libxkbcommon
      ] ++ pkgs.lib.lists.optionals stdenv.isDarwin [
        # [Raylib]
        # https://github.com/NixOS/nixpkgs/blob/master/pkgs/os-specific/darwin/apple-sdk/frameworks.nix
        darwin.apple_sdk.frameworks.Cocoa
        darwin.apple_sdk.frameworks.Kernel
      ];

      # Inputs needed at compile-time
      nativeBuildInputs = with pkgs; [ rustToolchain ];
      # Inputs needed at runtime
      buildInputs = with pkgs; [ ] ++ packages;
    in
    {
      packages.default = naerskLib.buildPackage {
         src = ./.;
      };

      devShells = {
        default = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputs ++ [
            pkgs.cargo-watch
          ];
        };
      };
    });
}
