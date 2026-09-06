{ pkgs, ... }:

{
  packages = [
    pkgs.diesel-cli
    pkgs.openssl
    pkgs.pkg-config
    pkgs.sqlite
    pkgs.rust-analyzer
    pkgs.ollama
  ];

  processes.ollama = {
    exec = "OLLAMA_HOST=127.0.0.1:11433 ${pkgs.ollama}/bin/ollama serve";
    ready = {
      http.get = {
        port = 11433;
        path = "/";
      };
      initial_delay = 1;
      period = 2;
      timeout = 60;
    };
  };

  languages.rust = {
    enable = true;
  };

  scripts.run.exec = ''
    cargo run
  '';
}
