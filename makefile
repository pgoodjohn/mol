.PHONY:release
release:
	cargo build --release --bin mol-cli
	mkdir -p ~/.mol/bin/
	mv -f ./target/release/mol-cli ~/.mol/bin/mol
	touch ~/.mol/conf.toml
