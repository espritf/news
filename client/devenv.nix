{ pkgs, ... }:

{
  packages = [
    pkgs.svelte-language-server
    pkgs.typescript-language-server
    pkgs.vscode-langservers-extracted
  ];

  languages.javascript = {
    enable = true;
    bun.enable = true;
  };

  scripts.run.exec = ''
    bun run dev
  '';
}
