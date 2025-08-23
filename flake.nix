{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [fenix.overlays.default];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          formatter = alejandra;
          devShells.default = mkShell rec {
            buildInputs = [
              pkgs.fenix.stable.toolchain
              lldb

              libGL
              libxkbcommon
              wayland
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
        }
    );
}
