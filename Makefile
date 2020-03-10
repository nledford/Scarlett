build:
	docker-compose build --pull

push:
	docker-compose push

run:
	docker-compose up -d --remove-orphans

logs:
	docker logs scarlett-server -f

# back up db

# restore db

cargo_fix:
	cargo fix --allow-dirty --allow-staged

cargo_sort:
	cargo-sort-ck -w

generate_certs:
	openssl genrsa 2048 > key.pem && \
		openssl req -x509 -days 1000 -new -key key.pem -out cert.pem