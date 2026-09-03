{ pkgs, ... }:

{
  packages = [
    pkgs.diesel-cli
    pkgs.openssl
    pkgs.pkg-config
    pkgs.sqlite
    pkgs.rust-analyzer
  ];

  languages.rust = {
    enable = true;
  };

  scripts.run.exec = ''
    cargo run
  '';
}
