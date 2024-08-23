
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustup
    gcc 
    rustfmt 
    clippy
    pkg-config
    libpqxx
    alsa-lib
    libudev-zero
    rust-analyzer
  ];
  buildInputs = [
    openssl
  ];
  
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  # Set Environment Variables
  RUST_BACKTRACE = 1;
  # ENV Variables
  LD_LIBRARY_PATH = "${geos}/lib:${gdal}/lib";
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
      with pkgs;
      lib.makeLibraryPath [ libGL libxkbcommon xorg.libX11 xorg.libXi xorg.libXcursor xorg.libXrandr vulkan-loader ]
    }"
  '';
}
    
