with import <nixpkgs> {};

# Uses Mozilla Rust Overlay
let
  crust = (rustChannels.stable.rust.override { extensions = [ "rust-src" ]; });
in
stdenv.mkDerivation {
  name = "usb-reset";
  buildInputs = [ crust rustracer pkg-config libusb ];
  RUST_SRC_PATH = "${crust}/lib/rustlib/src/rust/src";
}
