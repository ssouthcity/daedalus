{
  description = "Daedalus project flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }: 
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
          buildInputs = [
            # general
            pkgs.udev
            pkgs.alsa-lib
            pkgs.vulkan-loader
            # wayland
            pkgs.libxkbcommon
            pkgs.wayland
          ];
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
          ];
          
          LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
            # general
            pkgs.udev
            pkgs.alsa-lib
            pkgs.vulkan-loader
            # wayland
            pkgs.libxkbcommon
            pkgs.wayland
          ]}";
        };
      }
    );
}
