build:
	docker build -t nledford/scarlett-server .

push:
	docker push nledford/scarlett-server:latest

run_dev:
	docker run -it -p 8765:8765 \
		--env-file=.env \
		--mount type=bind,source=/Volumes/wd-beta/vault/Photos,target=/photos \
		--mount type=bind,source=/Users/nledford/Documents/Wallpaper,target=/wallpaper \
		nledford/scarlett-server

# run_dev

# back up db

# restore db

cargo_sort:
	cargo-sort-ck -w