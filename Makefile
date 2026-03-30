SOURCES=$(wildcard src/*.rs)


all: $(SOURCES) Makefile
	cargo build

commit:
	git add .
	aicommit
	git push

rebuild:
	make clean
	make all

test:
	cargo test -- --show-output

clean:
	cargo clean
	cargo update

all:
