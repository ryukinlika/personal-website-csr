# Personal Website with Rust Leptos (CSR) and Tailwind CSS

Starter CSR trunk were used to setup project and its initial configuration 
```sh
cargo generate --git https://github.com/leptos-community/start-trunk
```
Then, install tailwind css with npm 
```sh
npm install -D tailwindcss@4.1.2
```
and modify `Trunk.toml` to ensure Trunk is using the same version
```
[tools]
tailwindcss="4.1.12"
```
## Running the CSR webpage
For development, use
```sh
trunk serve --port 3000 --open
```
For Release, use 
```sh
trunk build --release
```
