# Anita

Anita is a comprehensive Rust project that provides a robust API server and various other services, including key management and storage solutions. The generated key will derive a public key for the specific suffix and will be stored in the database.

```sh
./anita -h

Work seamlessly with Anita from the command line

Usage: anita <COMMAND>

Commands:
  api   Start the API server
  key   Manage the keypairs
  db    Database tools
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

## Getting Started

### Prerequisites

- Rust and Cargo: Ensure you have the latest Rust toolchain installed on your system. You can install Rust and Cargo using [rustup](https://rustup.rs/).

### Installation

1. Build the project:

```bash
cargo build --release
```

For building a specific package, such as the API service, you can use:

```bash
cargo build --release -p anita-api
```

2. After the build completes, you can find the binary in `target/release/`. Make it executable:

```bash
cp target/release/anita .
chmod +x anita
```

### Usage

1. To start the API server, run:

```bash
Start the API server

Usage: anita api [OPTIONS] --database-url <database_url>

Options:
  -d, --database-url <database_url>  The database to save the keys [env: DATABASE_URL=]
  -p, --port <PORT>                  Number of threads to use [env: PORT=] [default: 3000]
  -h, --help                         Print help
  -V, --version                      Print version
```

2. To manager the secret key, run:

```bash
Manage the keypairs

Usage: anita key [OPTIONS] --database-url <database_url> <COMMAND>

Commands:
  get     Get a keypair
  new     New a keypair
  vanity  Vanity keypairs
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --database-url <database_url>  The database to save the keys [env: DATABASE_URL=]
  -s, --suffix <SUFFIX>              The suffix to search [default: sol]
  -h, --help                         Print help
  -V, --version                      Print version
```

3. To manager the db, run:

```bash
Database tools

Usage: anita db --database-url <database_url> <COMMAND>

Commands:
  migration  Execute database migrations
  version    Lists current and local database versions
  help       Print this message or the help of the given subcommand(s)

Options:
  -d, --database-url <database_url>  The database to save the keys [env: DATABASE_URL=]
  -h, --help                         Print help
  -V, --version                      Print version
```

Logs are output to the console and can also be found in the `logs/` directory.

## Development

### Building

Refer to the Installation section for details on building the project.

### Testing

Run the tests for the entire workspace with:

```bash
cargo test
```

### Adding New Dependencies

To add a new dependency, update the `Cargo.toml` file of the respective crate where you want to add the dependency.

## License

This project is licensed under either of MIT or Apache-2.0, at your option.

## Acknowledgments

- Diesel for database interactions ([diesel.toml](diesel.toml))
- Various Rust crates used throughout the project, as listed in [Cargo.lock](Cargo.lock)

For more information on configuring Diesel, see the [Diesel guide](https://diesel.rs/guides/configuring-diesel-cli).

## Contact

For any inquiries or contributions, please open an issue in the GitHub [repository](https://github.com/peng-huang-ch/anita-rs.git).
