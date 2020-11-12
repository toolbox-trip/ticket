ARCH = x86_64-unknown-linux-musl
VERSION = latest
NAME = ticket

toolchain:
	rustup target add $(ARCH)

debug: toolchain
	@cargo build

release: toolchain
	@cargo build --release --target=$(ARCH)

copy:
	cp ./target/$(ARCH)/$(NAME) ./docker/

docker: copy
	@echo PLEASE RUN `make release` FIRST
	@docker build --tag snail.azurecr.io/$(NAME):$(VERSION) ./docker
