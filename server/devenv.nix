{ pkgs, ... }:

let
  postgresql = pkgs.postgresql_16;
in
{
  packages = [
    pkgs.diesel-cli
    pkgs.openssl
    pkgs.pkg-config
    pkgs.ollama
    pkgs.rust-analyzer
    pkgs.sqls
  ];

  # onig_sys vendors an old oniguruma C source that fails to compile under
  # GCC's C23 default (gnu23 rejects the loose function-pointer initializers
  # in st.c). Force the older standard for all C deps built via the cc crate.
  # env.CFLAGS = "-std=gnu17";

  # diesel's pq-sys links against libpq but only locates it via pg_config or
  # PQ_LIB_DIR (it does not use pkg-config by default). Point it at libpq from
  # the same postgres package the service below runs.
  env.PQ_LIB_DIR = "${postgresql.lib}/lib";

  # Load .env into the shell so tools started from it (e.g. sqls in nvim) see
  # DATABASE_URL.
  dotenv.enable = true;

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
    package = postgresql;
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

  services.clickhouse = {
    enable = true;
    config = "";
  };

  scripts.server.exec = ''
    cargo run
  '';

  scripts.embedder.exec = ''
    cargo run --bin embedder
  '';
}
