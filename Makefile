VERSION = latest
NAME = ticket
BASE = localhost

debug:
	@cargo build

release:
	@cargo build --release

test:
	@cargo test --verbose

docker:
	@docker build --tag ${BASE}/$(NAME):$(VERSION) .
