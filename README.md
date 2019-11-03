# `rpiet`, the piet interpreter in Rust 🖼

[`piet`](http://www.dangermouse.net/esoteric/piet.html) is a esoteric programming language which executes image files.
This is a piet interpreter written in Rust.

![Composition with Red, Yellow and Blue. 1921, Piet Mondrian.](./doc/mondrian.jpg)
> Composition with Red, Yellow and Blue. 1921, Piet Mondrian.

Piet is a programming language which aims to treat images similar to those of the artist **Piet Mondrian** as executables.
Read more about how images are executed [at the Piet homepage](http://www.dangermouse.net/esoteric/piet.html).

## Installation and usage

Install `rpiet` via `cargo` (the Rust package manager). This requires an up-to-date Rust being installed.

    cargo install rpiet

Then run a GIF or PNG image with

    rpiet sample_images/hello_world_globe.png

or explore the command line options it takes with

    rpiet --help

It is possible to:

* specify the codel size (`-c`, `--codel-size <codel_size>`)
* limit the maximum number of steps the interpreter executes in the image (`-e`, `--max-steps <max_steps>`)
* print debugging information (`-v`, `--verbose`) which allows the user to see which path the interpreter takes through the image

## State of this crate

It is possible to run Piet programs in it (I verified a couple from the Piet homepage), please report any bugs you find - the specification is somewhat loose :)

Input handling is currently not strictly conform to the spec. We read lines of input -- the spec is not very detailed here, but I think we should read only the necessary bytes (InChar -> one byte, InNum -> however many bytes form a valid number, but no more).

The binary was tested manually, but we don't have automated tests yet. Also, code documentation is sparse.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/tessi/rpiet. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

## License

The crate is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the rpiet project’s codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/tessi/rpiet/blob/master/CODE_OF_CONDUCT.md).
