{
  description = "News client (Svelte + Vite)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        apps.default = {
          type = "app";
          program = "${pkgs.writeShellScript "dev" ''
            ${pkgs.bun}/bin/bun run dev
          ''}";
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.bun
            pkgs.nodejs
            pkgs.svelte-language-server
            pkgs.typescript-language-server
            pkgs.vscode-langservers-extracted
          ];
        };
      });
}
