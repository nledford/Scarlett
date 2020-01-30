build:
	docker build -t nledford/scarlett-server .

push:
	docker push nledford/scarlett-server:latest

# FOR PRODUCTION BUILD
# run

run_dev:
	docker run -it -p 8765:8765 \
		--name scarlett
		--env-file=.env \
		--mount type=bind,source=/Volumes/wd-beta/vault/Photos,target=/photos \
		--mount type=bind,source=/Users/nledford/Documents/Wallpaper,target=/wallpaper \
		nledford/scarlett-server

run_win:
	docker run -it -p 8765:8765 \
		--env-file=.env \
		--mount type=bind,source=//c/Users/nledford/Pictures/Vault,target=/photos \
		--mount type=bind,source=//c/Users/nledford/Pictures/Wallpaper,target=/wallpaper \
		nledford/scarlett-server

# back up db

# restore db

cargo_fix:
	cargo fix --allow-dirty --allow-staged

cargo_sort:
	cargo-sort-ck -w