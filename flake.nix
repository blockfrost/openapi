{
  description = "Blockfrost OpenAPI specification";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.nodejs_24
            pkgs.openjdk21
            (pkgs.yarn.override { nodejs = pkgs.nodejs_24; })
          ];

          shellHook = ''
            export PATH="$PATH:$(pwd)/node_modules/.bin"
          '';
        };
      });
}
