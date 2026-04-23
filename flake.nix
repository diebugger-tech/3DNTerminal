{
  description = "3DNTerminal - Rust Terminal for COSMIC Desktop";

  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust
            pkg-config
            gcc
            cmake
            wayland
            wayland-protocols
            libxkbcommon
            vulkan-loader
            vulkan-headers
            mesa
            fontconfig
            freetype
            openssl
          ];

          shellHook = ''
            echo "🦀 3DNTerminal Dev-Shell aktiv"
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.vulkan-loader
              pkgs.mesa
              pkgs.libxkbcommon
            ]}:$LD_LIBRARY_PATH"
          '';
        };
      }
    );
}
