.PHONY: cli
cli:
	(cargo build ; cp target/debug/shit shit)