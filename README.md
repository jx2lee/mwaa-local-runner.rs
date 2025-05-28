# mwaa-local-runner.rs

<img
    src="https://github.com/jx2lee/mwaa-local-runner.rs/blob/main/assets/logo.png?raw=true"
    alt="mwaa-local-runner.rs logo"
    width="300"
/>

mwaa-local-runner.rs is a command-line tool for working with an Apache Airflow environment locally. It is designed to provide a local workflow that resembles Amazon Managed Workflows for Apache Airflow (Amazon MWAA).

## Overview

Use this project to build and run a Docker-based Airflow environment, validate local prerequisites, test dependency files, and open an Amazon MWAA web login URL from the command line.

## Requirements

The local environment expects the following tools to be available:

- Docker
- Docker Compose
- Python 3
- pip
- AWS CLI, when using AWS authentication or SSO flows

You can check the required local commands with:

```sh
cargo run -- validate-prereqs
```

## Usage

Run a command with Cargo:

```sh
cargo run -- <command>
```

By default, the CLI uses Airflow `2.10.1`. To select another version, pass `--airflow-version` before the command:

```sh
cargo run -- --airflow-version 2.10.1 build-image
```

## Commands

| Command | Description |
| --- | --- |
| `build-image` | Build the local Airflow Docker image. |
| `start` | Start the local Airflow environment with LocalExecutor and PostgreSQL. |
| `reset-db` | Reset the local PostgreSQL container. |
| `test-requirements` | Install requirements in an ephemeral container instance. |
| `package-requirements` | Download requirement wheel files into the plugins folder. |
| `test-startup-script` | Run a startup script in an ephemeral container instance. |
| `validate-prereqs` | Check whether required local commands are installed. |
| `login-web` | Create an Amazon MWAA web login URL and open it in a browser. |

For command-specific options, use:

```sh
cargo run -- <command> --help
```

### Open an MWAA web login URL

Use `login-web` with an MWAA environment name:

```sh
cargo run -- login-web --mwaa-env <environment-name>
```

To use a named AWS profile:

```sh
cargo run -- login-web \
  --mwaa-env <environment-name> \
  --aws-profile <profile-name>
```

To run AWS SSO login before creating the web login token:

```sh
cargo run -- login-web \
  --mwaa-env <environment-name> \
  --aws-profile <profile-name> \
  --with-sso
```

The default AWS Region is `ap-northeast-2`. Override it with `--aws-region`.

## Development

Format the code:

```sh
just fmt
```

Run Clippy:

```sh
just clippy
```

Run tests:

```sh
just test
```

## License

This project is licensed under the MIT License. See [LICENCE](LICENCE) for details.
