{ pkgs, ... }:

{
  packages = [
    pkgs.udev
    pkgs.alsa-lib-with-plugins
    pkgs.vulkan-loader

    # x11
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXi

    # wayland
    pkgs.libxkbcommon
    pkgs.wayland

    # pkgs.libudev0-shim
    # pkgs.libudev-zero

    # Level Design
    pkgs.ldtk
    pkgs.aseprite
  ];

  languages.rust = {
    enable = true;
    mold.enable = true;
  };

  env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.libxkbcommon
    pkgs.vulkan-loader
    pkgs.wayland
  ];

  processes = {
    ldtk.exec = "ldtk";
    aseprite.exec = "aseprite";
  };

  git-hooks.hooks = {
    # General
    trim-trailing-whitespace.enable = true;
    end-of-file-fixer.enable = true;

    # Rust
    cargo-check.enable = true;
    clippy.enable = true;
    rustfmt.enable = true;

    # Git
    check-merge-conflicts.enable = true;
    no-commit-to-branch.enable = true;
  };
}
