with import <nixpkgs> {};

mkShell {
  buildInputs = [
    cargo
    rustc
    fish
  ] ++
  (stdenv.lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.IOKit
    xcbuild
  ]);
}
