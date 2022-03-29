.PHONY:release
release:
	cargo build --release --bin mol-cli
	mkdir -p ~/.mol/bin/
	mv -f ./target/release/mol-cli ~/.mol/bin/mol
	[ ! -f ~/.mol/conf.toml ] && cp ./release/config/sample.toml ~/.mol/conf.toml
