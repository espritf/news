include .env

DB_PATH := $(patsubst sqlite://%,%,$(DATABASE_URL))

create:
	test -f $(DB_PATH) || (mkdir -p var && diesel migration run --database-url=$(DATABASE_URL))

clean:
	sqlite3 $(DB_PATH) 'delete from items'

drop:
	rm -f $(DB_PATH)

reset-republish:
	sqlite3 $(DB_PATH) 'update items set published_id = null'

show:
	sqlite3 $(DB_PATH) 'select * from items'

create-translator:
	ollama show translator >/dev/null 2>&1 || ollama create translator -f Modelfile

build: create create-translator
