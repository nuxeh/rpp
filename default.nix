with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "url-bot-rs";

    buildInputs = [
      rustc
      cargo
    ];
}
