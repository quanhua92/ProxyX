# ProxyX - Another Experimental Proxy Written in Rust

## Development

- Install `pre-commit` and set up Git hooks

  ```bash
  pip install pre-commit
  pre-commit install
  pre-commit run --all-files
  ```

- Install `cargo-watch`, `bunyan`

  ```bash
  cargo install cargo-watch bunyan
  ```

- Start the server

  ```bash
  cargo watch -x run
  ```

- Run test with tracing

  ```bash
  TEST_LOG=1 cargo test | bunyan
  ```
