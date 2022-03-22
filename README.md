# Mol
A CLI interface for the Mollie API

---

## Installation
Installing the `mol` CLI interface will require you to build it from source. To do this you will need `cargo` installed on your machine. You can verify that it is installed by running:

```
$ cargo -V  
cargo 1.59.0 (49d8809dc 2022-02-10)
```

If you don't have `cargo` installed, you can set it up with:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once you have `cargo` ready to go, you can build `mol` by running

```
make release
```

This will create a release build, move it to `~/.mol/bin/mol` and create an empty `~/.mol/conf.toml` file for the application to use.

To be able to run `mol` from anywhere, simply add the following to your `.zshrc`

```
if [ -f "/Users/$USERNAME/.mol/bin/mol" ]; then
    path+=("/Users/$USERNAME/.mol/bin/mol")
    export PATH
fi
```

### Update
To update to a newerversion of `mol`, simply run `make release` again. This will rebuild the binary with the latest code in your local repository and move it to `~/.mol/bin/mol`.