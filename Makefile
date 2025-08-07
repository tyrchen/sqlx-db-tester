build:
	@cargo build

cov:
	@cargo llvm-cov nextest --all-features --workspace --lcov --output-path coverage/lcov-$(shell date +%F).info

test:
	@CELLA_ENV=test cargo nextest run --all-features

test-mysql:
	@cargo test --features mysql -- --nocapture

test-postgres:
	@cargo test --features postgres -- --nocapture

test-all: mysql-start postgres-start
	@cargo test --all-features -- --nocapture
	@$(MAKE) mysql-stop
	@$(MAKE) postgres-stop

# MySQL Docker/Podman commands
mysql-start:
	@echo "Starting MySQL container..."
	@podman run -d \
		--name sqlx-mysql-test \
		-e MYSQL_ROOT_PASSWORD=password \
		-e MYSQL_DATABASE=test \
		-p 3307:3306 \
		mysql:8.0 \
		--default-authentication-plugin=mysql_native_password \
		--local-infile=1 \
		|| echo "MySQL container already running"
	@echo "Waiting for MySQL to be ready..."
	@sleep 5
	@until podman exec sqlx-mysql-test mysqladmin ping -h localhost -u root -ppassword --silent; do \
		echo "Waiting for MySQL..."; \
		sleep 2; \
	done
	@echo "MySQL is ready!"

mysql-stop:
	@echo "Stopping MySQL container..."
	@podman stop sqlx-mysql-test || true
	@podman rm sqlx-mysql-test || true

mysql-logs:
	@podman logs sqlx-mysql-test

# PostgreSQL Docker/Podman commands
postgres-start:
	@echo "Starting PostgreSQL container..."
	@podman run -d \
		--name sqlx-postgres-test \
		-e POSTGRES_PASSWORD=postgres \
		-e POSTGRES_USER=postgres \
		-e POSTGRES_DB=postgres \
		-p 5432:5432 \
		postgres:16 \
		|| echo "PostgreSQL container already running"
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	@until podman exec sqlx-postgres-test pg_isready -U postgres; do \
		echo "Waiting for PostgreSQL..."; \
		sleep 2; \
	done
	@echo "PostgreSQL is ready!"

postgres-stop:
	@echo "Stopping PostgreSQL container..."
	@podman stop sqlx-postgres-test || true
	@podman rm sqlx-postgres-test || true

postgres-logs:
	@podman logs sqlx-postgres-test

# Clean all containers
clean-containers:
	@$(MAKE) mysql-stop
	@$(MAKE) postgres-stop

release:
	@cargo release tag --execute
	@git cliff -o CHANGELOG.md
	@git commit -a -n -m "Update CHANGELOG.md" || true
	@git push origin master
	@cargo release push --execute

.PHONY: build cov test test-mysql test-postgres test-all mysql-start mysql-stop mysql-logs postgres-start postgres-stop postgres-logs clean-containers release
