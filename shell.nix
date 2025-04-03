with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "openapi";
  buildInputs = [
    nodejs_20
    pkgs.openjdk21
    (yarn.override { nodejs = nodejs_20; })
  ];
  shellHook = ''
    export PATH="$PATH:$(pwd)/node_modules/.bin"
    yarn
    echo "Rebuilding docs"
    yarn build
    yarn run build
    echo -e "\nDON'T FORGET TO RUN yarn build after changing files"
  '';
}
