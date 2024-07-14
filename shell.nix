with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "openapi";
  buildInputs = [
    nodejs_20
    (yarn.override { nodejs = nodejs_20; })
  ];
  shellHook = ''
    export PATH="$PATH:$(pwd)/node_modules/.bin"
    yarn
    echo "Rebuilding docs"
    yarn build
    yarn run bundle && yarn run generate-docs && yarn run generate-types
    echo -e "\nDON'T FORGET TO RUN yarn build && yarn run bundle && yarn run generate-docs && yarn run generate-types"
    echo "yarn build && yarn run bundle && yarn run generate-docs && yarn run generate-types"
  '';
}
