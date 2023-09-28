.PHONY: start-db
start-db:
	podman-compose -f deployment/db/db.yml up --detach

.PHONY: stop-db
stop-db:
	podman-compose -f deployment/db/db.yml down

.PHONY: run-migrations
run-migrations:
	export PGPASSWORD=password && \
	until psql -h "localhost" -U "postgres" -p "5432" -d "postgres" -c '\q'; do \
		>&2 echo "Postgres is still unavailable - sleeping"; \
		sleep 1; \
	done

	@sqlx migrate run --database-url postgres://postgres:password@127.0.0.1:5432/tard_fi_data