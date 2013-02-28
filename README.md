# rust-airhockey

This is a game I created for #1GAM to try, learn and potentially show others the Rust language. It uses rust-sdl and has its own OpenGL bindings. The game itself isn't that great, but it is the most basic form of airhockey I could think of.

This project might be of use for others: it shows handling structs (Paddle, Puck), boxed pointers, traits (GameObject), operator overloading (vec2) and generally how to set up a game using rust-sdl and OpenGL.

## Usage

rust-airhockey requires a very recent version of the Rust compiler. I used rustc 0.6 (b171d0e 2013-02-28 01:12:38 -0800) to compile the project. At the time of writing this version is in the [incoming](https://github.com/mozilla/rust/tree/incoming) branch of Rust.

To compile rust-airhockey first install [rust-sdl](https://github.com/brson/rust-sdl):

    $ cargo install sdl

Now you can compile rust-airhockey:

    $ rustc main.rs

To run rust-airhockey:

    $ ./main
