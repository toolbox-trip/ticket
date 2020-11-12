VERSION = latest
NAME = ticket
BASE = snail.azurecr.io

debug:
	@cargo build

release:
	@cargo build --release

test:
	@cargo test --verbose

docker: test release copy
	@docker build --tag ${BASE}/$(NAME):$(VERSION) ./docker
