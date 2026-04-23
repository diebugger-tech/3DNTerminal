{
  description = "3DNTerminal";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { inherit system; };
        runtimeLibs = with pkgs; [
          wayland libxkbcommon vulkan-loader libGL mesa
          libx11 libxcursor libxi libxrandr
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo rustc pkg-config gcc openssl freetype wayland-protocols
            ghostty
          ] ++ runtimeLibs;
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"
            export WINIT_UNIX_BACKEND=wayland
            echo "🦀 3DNTerminal + Ghostty ready"
          '';
        };
      }
    );
}
