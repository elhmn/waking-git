.DEFAULT_GOAL := help

WAKE_FOLDER = .wake
BIN_PATH = ~/$(WAKE_FOLDER)/bin
PLAYERS_BIN_PATH = ~/$(WAKE_FOLDER)/bin/players
PLAYERS_TARGET = ./target/debug/players
WAKE_TARGET = ./target/debug/wake

## build: build application binary.
.PHONY: build
build: install-players
	cargo build

## run: run an example.
.PHONY: run
run: install-players
	cargo run -p wake -- play shmup https://github.com/osscameroon/osscameroon-website

## build-wake: build wake binary.
.PHONY: build-wake
build-wake: $(WAKE_TARGET)

$(WAKE_TARGET):
	cargo build -p wake

## install-players: install players binary.
.PHONY: install-players
install-players: $(PLAYERS_BIN_PATH)

$(PLAYERS_BIN_PATH):
	mkdir -p ~/$(WAKE_FOLDER)/bin/
	cp $(PLAYERS_TARGET) $(BIN_PATH)

## build-players: build players binary.
.PHONY: build-players
build-players: $(PLAYERS_TARGET)

$(PLAYERS_TARGET):
	cargo build -p players

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

## lint-players: run linter over the players workspace
.PHONY: lint
lint-players: install-deps
	cargo clippy -p players -- -D warnings || (echo "\nFix linting errors with \`__CARGO_FIX_YOLO=1 cargo clippy --fix\`"; exit 1)

## lint-wake: run linter over the wake workspace
.PHONY: lint
lint-wake: install-deps
	cargo clippy -p wake -- -D warnings || (echo "\nFix linting errors with \`__CARGO_FIX_YOLO=1 cargo clippy --fix\`"; exit 1)

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
	@echo " or run an example using \`make run\`"
	@echo ""
	@echo " Choose a command..."
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo ""
	@echo "You could run it using cargo commands directly"
	@echo ""
	@echo "Make sure to build and install the player before running it:"
	@echo "\`"make build-players \; make install-players"\`"
	@echo ""
	@echo "Then run: \`"cargo run -p wake -- play shmup https://github.com/elhmn/waking-git"\`"
	@echo ""
	@echo "Scan a repo:"
	@echo "\`"cargo run -p wake -- scan https://github.com/elhmn/waking-git"\`"
	@echo ""
	@echo "Run the player:"
	@echo "\`"cargo run -p players -- shmup ~/.wake/scanner/github-com-elhmn-waking-git/shmup-converted.json"\`"
