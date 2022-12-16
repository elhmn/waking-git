.DEFAULT_GOAL := help

## build: build application binary.
.PHONY: build
build:
	cargo build

.PHONY: install-deps
install-deps:
ifeq (, $(shell which cargo))
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
endif

## test: run tests
.PHONY: test
test: install-deps
	cargo test -- --test-threads 1

## lint: run linter over the entire code base
.PHONY: lint
lint: install-deps
	cargo clippy -- -D warnings || (echo "\nFix linting errors with \`__CARGO_FIX_YOLO=1 cargo clippy --fix\`"; exit 1)

## fmt: check your code format
.PHONY: fmt
fmt: install-deps
	cargo fmt --check || (echo "\nFix formatting errors with \`cargo fmt\`"; exit 1)

## install-hooks: install local git hooks
.PHONY: install-hooks
install-hooks:
	ln -s $(PWD)/githooks/pre-push .git/hooks/pre-push

.PHONY: all
all: help

.PHONY: help
help: Makefile
	@echo " You can build \`wake\` using \`make build\`"
	@echo " or run it using \`cargo run scan https://github.com/elhmn/ckp\`"
	@echo ""
	@echo " Choose a command..."
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
