with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "openapi";
  buildInputs = [
    nodejs-14_x
    (yarn.override { nodejs = nodejs-14_x; })
  ];
  shellHook = ''
    export PATH="$PATH:$(pwd)/node_modules/.bin"
    npm install
    echo "To build docs run"
    echo "> npm run build"
  '';
}
