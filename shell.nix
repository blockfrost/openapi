with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "openapi";
  buildInputs = [
    nodejs-20_x
    (yarn.override { nodejs = nodejs-20_x; })
  ];
  shellHook = ''
    export PATH="$PATH:$(pwd)/node_modules/.bin"
    yarn
    echo "Rebuilding docs"
    yarn run bundle && yarn run generate-docs && yarn run generate-types
    echo -e "\nDON'T FORGET TO RUN THIS BEFORE OPENING PR:"
    echo "yarn run bundle && yarn run generate-docs && yarn run generate-types"
  '';
}
