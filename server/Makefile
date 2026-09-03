include .env

create:
	diesel migration run --database-url=$(DATABASE_URL)

clean:
	psql -d $(POSTGRES_DB) -c "delete from news;"

drop:
	PGPASSWORD=$(POSTGRES_PASSWORD) dropdb -h localhost -U $(POSTGRES_USER) $(POSTGRES_DB)

redo:
	diesel migration redo --database-url=$(DATABASE_URL)
	
show:
	psql -d $(POSTGRES_DB) -c "select * from news;"
