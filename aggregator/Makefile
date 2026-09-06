include .env

create:
	mkdir -p var
	diesel migration run --database-url=$(DATABASE_URL)

clean:
	sqlite3 var/aggregator.db 'delete from items'

drop:
	rm -f aggregator/var/aggregator.db

reset-republish:
	sqlite3 var/aggregator.db 'update items set published_id = null'

show:
	sqlite3 var/aggregator.db 'select * from items'

create-translator:
	ollama create translator -f Modelfile
