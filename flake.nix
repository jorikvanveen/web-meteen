{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          nodejs_22
          nodePackages.pnpm
          nodePackages.eslint
          nodePackages.svelte-language-server
          tailwindcss-language-server
          wasm-pack
        ];
      };
    }
  );
}
