let
   pkgs = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation rec {
  name = "rust-bevy";
  nativeBuildInputs = [ pkgs.pkg-config ];
  buildInputs = with pkgs; [
    rust-analyzer

    xorg.libxcb
    xorg.libX11

    vulkan-tools
    vulkan-loader
    vulkan-headers
    vulkan-validation-layers

    alsaLib
    udev
  ];
}
