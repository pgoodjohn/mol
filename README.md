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

## Usage

After running `make release` and reloading your shell you can start using `mol` right away:

```
$ mol

mol-cli 0.1.0
A Command Line Interface for the Mollie API

USAGE:
    mol [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -d, --debug
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    auth        Do Auth things
    help        Print this message or the help of the given subcommand(s)
    org         Do Organizationy things
    payments    Do things with Payments
```

Every command will print the above help menu if not enough argument are provided (or if it is run with the `--help` flag).

### Authenticate

To interact with the Mollie API, you will need to register an API Key. You can either generate an "Organization Access Code" or get your "Live" or "Test" API keys from the [Mollie Dashboard](https://my.mollie.com/dashboard/developers/api-keys). 

Once you obtained one, you can register it with `mol` by running:

```
$ mol auth add --api-key {live_123134123} # Live API Key
$ mol auth add --api-key {test_123134123} # Test API Key
$ mol auth add --access-code {access_1231231123} # Organization Access Token
```

To verify you are authenticated correctly, you can get your organization details with:

```
$ mol auth get
```

And you can verify the permissions of your token with:

```
$ mol auth get permissions
```

### Payments

`mol` can help you check, create and refund payments through the Mollie API. You can do so with the `mol payments` command:

```
$ mol payments
mol-payments 0.1.0
Do things with Payments

USAGE:
    mol payments [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -d, --debug
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    create    Create a new payment
    get       Get a payment's info
    help      Print this message or the help of the given subcommand(s)
    list      List payments
    refund    Refund a payment
```