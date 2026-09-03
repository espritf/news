{ pkgs, ... }:

{
  packages = [
    pkgs.diesel-cli
    pkgs.openssl
    pkgs.pkg-config
    pkgs.ollama
    pkgs.rust-analyzer
  ];

  # onig_sys vendors an old oniguruma C source that fails to compile under
  # GCC's C23 default (gnu23 rejects the loose function-pointer initializers
  # in st.c). Force the older standard for all C deps built via the cc crate.
  # env.CFLAGS = "-std=gnu17";

  languages.rust = {
    enable = true;
  };

  processes.ollama = {
    exec = "${pkgs.ollama}/bin/ollama serve";
    ready = {
      http.get = {
        port = 11434;
        path = "/";
      };
      initial_delay = 1;
      period = 2;
      timeout = 60;
    };
  };

  services.postgres = {
    enable = true;
    package = pkgs.postgresql_16;
    extensions = extensions: [ extensions.pgvector ];
    listen_addresses = "localhost";
    port = 5432;
    initialDatabases = [
      {
        name = "news";
        user = "postgres";
        pass = "postgres";
        initialSQL = "CREATE EXTENSION IF NOT EXISTS vector;";
      }
    ];
  };

  scripts.run.exec = ''
    cargo run
  '';
}
