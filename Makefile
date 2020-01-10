build:
	docker build -t nledford/scarlett-server .

push:
	docker push nledford/scarlett-server:latest

run_dev:
	docker run -p 8765:8765 \
		--env-file=.env \
		--mount type=bind,source=/Volumes/wd-beta/vault/Photos,target=/photos \
		nledford/scarlett-server

# run_dev

# back up db

# restore db