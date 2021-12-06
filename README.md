# fmgt Web Server
`fmgt` is a lightweight webserver, written in Rust using the Rocket framework, providing
HTTP APIs for basic file management tasks. The server includes endpoints for listing file paths,
uploading and downloading files. An OpenAPI v3 specification is included.

## Building the Project
The project requires a recent version of Rust (e.g., 1.57.0); install using `rustup`. With rust
installed, use `cargo` to build the project:

```
$ cargo build
```

or run a development server

```
$ cargo run
```

## Examples

The following examples use `curl` to illustrate the functionality.




