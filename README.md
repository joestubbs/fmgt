# fmgt Web Server
`fmgt` is a lightweight webserver, written in Rust using the Rocket framework, providing
HTTP APIs for basic file management tasks. The server includes endpoints for listing file paths,
uploading and downloading files. When built with `--release` on Linux, the resulting binary is ~8MB.

An OpenAPI v3 specification is included.

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


1. List the files at the path `/home/jstubbs/projects/rust/tmp`:

```
$ curl localhost:8000/list/home/jstubbs/projects/rust/tmp | jq

{
  "status": "success",
  "message": "file listing returned suceesfully",
  "result": [
    "hello",
    "test"
  ],
  "meta": "none",
  "version": "0.1.0"
}
```

2. Upload a file `testup` with contents "this is a test" to the `/home/jstubbs/projects/rust/tmp` directory:

```
$ curl -d @test localhost:8000/contents/home/jstubbs/projects/rust/tmp/testup | jq

{
  "status": "success",
  "message": "file uploaded suceesfully",
  "result": "",
  "meta": "none",
  "version": "0.1.0"
}
```

3. Check that the new `testup` file now shows up in the listing:

```
$ curl localhost:8000/list/home/jstubbs/projects/rust/tmp | jq

{
  "status": "success",
  "message": "file listing returned suceesfully",
  "result": [
    "hello",
    "test",
    "testup"
  ],
  "meta": "none",
  "version": "0.1.0"
}
```


4. Stream down the contents of the `testup` file we just uploaded:

```
$ curl localhost:8000/contents/home/jstubbs/projects/rust/tmp/testup
this is a test

