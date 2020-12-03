# Use latest stable Rust
#
# To use, clone the Mozilla overlay, and provide the path at nix-shell
# invocation, e.g.:
#
#     git clone https://github.com/mozilla/nixpkgs-mozilla.git
#     nix-shell stable.nix -I rustoverlay=/path/to/overlay
#
# If you want to update Rust versions to the nightly builds, just pull the
# local overlay repository.

with import <nixpkgs> {};
with import <rustoverlay/rust-overlay.nix> pkgs pkgs;

stdenv.mkDerivation {
  name = "rpp";

  buildInputs = [
    latest.rustChannels.stable.rust
  ];
}
