<div align="center">

  <h1><code>WASM Search widget</code></h1>

  <p><strong>Rust Language and its Ecosystem TX00FL66-3001</strong></p>

##About

Use wasm-pack to compile a Rust method that is used to search for a kanji found in the list of Joyo Kanji, by matching its stroke count, and then display the results on a web page.

### Usage

### Install wasm-pack with cargo or check [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
```sh
cargo install wasm-pack
```

###  Build with wasm-pack
```sh
wasm-pack build --target web
```

###  Run with (for example) python server or http-server
```sh
python3 -m http.server
```

###  Run tests in browser 
```sh
wasm-pack test --headless --firefox
```

# Image of app running in localhost
![view in localhost](<Screenshot 2024-02-15 at 9.38.23.png>)

# Image of testrun
![test result](<Screenshot 2024-02-15 at 11.43.29.png>)