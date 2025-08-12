.PHONY: all run clean

all: run

run:
	@cargo run

clean:
	@cargo clean
	@clear