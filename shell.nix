let
   pkgs = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation rec {
  name = "rust-bevy";
  nativeBuildInputs = [ pkgs.pkg-config ];
  WGPU_BACKEND = "vulkan";
  WGPU_POWER_PREF = "high";
  buildInputs = with pkgs; [
    rust-analyzer

    xorg.libxcb
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi

    vulkan-tools
    vulkan-loader
    vulkan-headers
    vulkan-validation-layers

    alsaLib
    udev
  ];
  LD_LIBRARY_PATH = "${pkgs.stdenv.lib.makeLibraryPath buildInputs}";
}
