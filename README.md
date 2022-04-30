## Mini-Bank Frontend
The frontend for the Mini-Bank project with logic entirely written in Rust using Sycamore, wasm-bindgen and web-sys crates.

#### CSS styling
The UI uses FrowCSS, which is a small flexbox css library that is easy to learn.

#### WebAssembly build tools
- The wasm32-unknown-unknown toolchain is used as the WebAssembly target. This allows writing pure Rust apps using any Rust frontend framework.
- [Trunkrs](https://trunkrs.dev) is used to automate building the WebAssembly files and generating JavaScript bindings to load the WebAssembly modules into the browser.

#### Running the UI

1. Follow the installation instructions as indicated on the sycamore website - https://sycamore-rs.netlify.app/docs/getting_started/installation

2. Run trunk

   ```sh
   $ trunk serve
   ```
