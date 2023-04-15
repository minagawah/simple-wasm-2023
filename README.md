# very-simple-wasm-2023

## 1. About

A sample WASM app using
[wasm-pack-plugin](https://github.com/wasm-tool/wasm-pack-plugin).  
It is prepared as a learning material for internal use.

I have another project,
[flight-pack](https://github.com/minagawah/flight-pack),
which also uses `wasm-pack-plugin`. Although it is a bit complicated,
it has practical examples, and probably has better explanations for program flows.

I have another simple WASM sample app, called
[perlin-experiment-2](https://github.com/minagawah/perlin-experiment-2).
It is a matter of preference, but you may find it simpler
for the way of spawning a child process.

Or, if you want to directly use `wasm-pack` instead of using `wasm-pack-plugin`,
while it is a bit old, but I have
[perlin-experiment](https://github.com/minagawah/perlin-experiment)
which also has good examples using Rust's _traits_.

Since we heavily depend on
[web-sys](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html)
as we develop WASM apps for front-end, it is also a good idea
to spend some time reading their documents as well.

## 2. Instructions

### 2-1. Install Rust + wasm-pack

#### # Rust

Install Rust. See details in [here](https://www.rust-lang.org/tools/install).
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
For mine, I have `v1.63.0`:

```bash
$ rustc --version
rustc 1.63.0 (4b91a6ea7 2022-08-08)
```

#### # wasm-pack

Install `wasm-pack`. See details in [here](https://rustwasm.github.io/wasm-pack/installer/).
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Also, you'd better [check out their documentations](https://rustwasm.github.io/docs/wasm-pack/introduction.html)
for they have awesome writings on developing WASM apps.

#### # clippy

In case you have not yet installed
[clippy](https://github.com/rust-lang/rust-clippy),
you may want to install it.
It is a handy formatting tool for Rust codes.

```bash
rustup component add clippy
```

### 2-2. Install NPM Packages + Building WASM

1. Clone this repo
1. `npm install`
1. (for MacOS: `npm run clippy`)
1. `npm run dev`

Under the hood, it is executing `wasm-pacck build`.
When it successfully compile Rust codes,
it will create new directories `target` and `pkg`.

__For MacOS users:__
Although it was totally fine with my Ubuntu envirnment,
nothing happened when I ran `npm run dev` on my MacBook.
If you encouter the same, then you may want to
try running `npm run clippy` first.
By doing this, it will start downloading necessary crates.
Once you are finished, try running `npm run clippy` again.


## 3. What I did

### 3-1. Rust + wasm-pack

Installed Rust + wasm-pack.

### 3-2. NPM Packages

#### Babel
- @babel/cli
- @babel/core
- @babel/preset-env
- @babel/runtime-corejs3
- core-js
- babel-loader

#### Webpack & Loaders
- webpack
- webpack-cli
- webpack-dev-server
- file-loader
- css-loader
- postcss-loader
- style-loader
- clean-webpack-plugin
- html-webpack-plugin
- autoprefixer

#### Others
- rimraf
- prettier
- @wasm-tool/wasm-pack-plugin

#### All NPM Packages
```bash
npm install --save-dev @babel/cli @babel/core @babel/preset-env @babel/runtime-corejs3 core-js babel-loader webpack webpack-cli webpack-dev-server file-loader css-loader postcss-loader style-loader clean-webpack-plugin html-webpack-plugin autoprefixer rimraf prettier @wasm-tool/wasm-pack-plugin
```

## 3. License

Dual-licensed under either of the followings.  
Choose at your option.

- The UNLICENSE ([LICENSE.UNLICENSE](LICENSE.UNLICENSE))
- MIT license ([LICENSE.MIT](LICENSE.MIT))
