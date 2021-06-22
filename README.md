# Rust-Checkers

This template will help you to build a checkers game in Rust.
I am using -

- Rust to write the game engine code
- JavaScript to host the WebAssembly Module
- Wasm as compile target for Rust code

## Building the project

- Run the command to generate the targer wasm module from Rust code.
  ```
  cargo build --release --target wasm32-unknown-unknown
  ```
- Copy the compiled wasm module to demo folder where our JS and html files are located.
  ```
  cp target/wasm32-unknown-unknown/release/rustycheckers.wasm demo/
  ```
## Run the project

- Run the python server in demo project.
  ```
  python3 -m http.server
  ```
  
Now you are good to go, open the console at 8000 port in your browser and see your checkers game running.

