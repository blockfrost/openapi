with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "openapi";
  buildInputs = [
    nodejs-12_x
    yarn
  ];
  shellHook = ''
    export PATH="$PATH:$(pwd)/node_modules/.bin"
  '';
}
