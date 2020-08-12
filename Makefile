CC = cargo build
browser = firefox
doc = target/doc/rustlativity/index.html


build: build-debug build-release build-doc

build-debug:
	$(CC)

build-release:
	$(CC) --release

build-doc:
	cargo doc

read-doc:
	$(browser) file:target/doc/rustlativity/index.html
