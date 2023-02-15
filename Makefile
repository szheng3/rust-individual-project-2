PARAMETER = default

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

format-check:
	cargo check

runarm:
	export LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1 &&export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH&&cargo run -- text -i "$(PARAMETER)"

bench:
	export LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1 &&export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH&&cargo bench -- text -i "$(PARAMETER)"

run:
	cargo run -- text -i "$(PARAMETER)"

release:
	export LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1 &&export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH&&cargo build --release

bench:
	export LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1 &&export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH&&cargo bench

benchx86:
	cargo bench

releasex86:
	cargo build --release

all: format lint test run
