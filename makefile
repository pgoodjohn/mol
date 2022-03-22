.PHONY:release
release:
	cargo build --release --bin mol
	mkdir -p ~/.mol/bin/
	mv -f ./target/release/mol ~/.mol/bin/mol
	touch ~/.mol/conf.toml
