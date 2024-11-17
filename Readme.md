# URL Shortener

Service for shortening URLs using **CQRS (Command Query Responsibility Segregation)** and **ES (Event Sourcing)**
architectural approaches.

## Features

- **Short link creation:**
    - Generate a short link with a random slug.
    - Create a short link with a predefined slug.
- **Redirect tracking:**
    - Count the number of redirects for each short link.
    - Retrieve redirect statistics via API.
- **Event sourcing:**
    - All state changes (link creation, redirect counts) are recorded as events.
    - Events can be replayed to reconstruct the system's state.
- **File-based storage:**
    - Slugs, links, and statistics are stored locally in a file system.

## API Documentation

Visit [`localhost:3000/scalar`](http://localhost:3000/scalar) after starting the service to see the full API
documentation generated with **Utoipa**.

## Usage

### Running the Service

To run the service locally:

```bash
cargo run
```

The server will be available at `http://localhost:3000`.

## Building for Release

To build an optimized binary for production:

```bash
cargo build --release
```

## Development Notes

## CQRS + ES Architecture

### CQRS (Command Query Responsibility Segregation):

- Commands (e.g., creating links, updating redirect counts) are separated from queries (e.g., retrieving link
  statistics).

### Event Sourcing:

- All state changes are recorded as events, which can be replayed to reconstruct the current state of the system.

## File-Based Storage

- Data is stored in events.json.
- On server shutdown, all in-memory state is saved to the file.
- The file is replayed to restore state when the server restarts.

## Linter

To run the linter:

```bash
cargo clippy
```

Instructions for installing it can be found [here](https://github.com/rust-lang/rust-clippy).

# Limitations

This implementation uses a file-based approach for storage, making it suitable for small applications. For larger
systems, consider using a database (such as PostgreSQL or MongoDB).

`TODO: maybe I'll add this later`

[//]: # (`TODO: maybe I'll add this later`)

# License

[MIT](LICENSE)
