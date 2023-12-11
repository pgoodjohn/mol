.PHONY:release
release:
	cargo build --release
	mkdir -p ~/.mol/bin/
	cp -f ./target/release/mol ~/.mol/bin/mol
	[ ! -f ~/.mol/conf.toml ] && cp ./release/config/sample.toml ~/.mol/conf.toml