# `rpiet`, the piet interpreter in Rust 🖼

[`piet`](http://www.dangermouse.net/esoteric/piet.html) is a esoteric programming language which executes image files.
This is a piet interpreter written in Rust.

![Composition with Red, Yellow and Blue. 1921, Piet Mondrian.](./doc/mondrian.jpg)
> Composition with Red, Yellow and Blue. 1921, Piet Mondrian.

Piet is a programming language which aims to treat images similar to those of the artist **Piet Mondrian** as executables.
Read more about how images are executed [at the Piet homepage](http://www.dangermouse.net/esoteric/piet.html).

## State of this crate

It is possible to run some Piet programs in it (I verified a couple of hello world images from the Piet homepage), please report any bugs you find - the specification is somewhat loose :)

"Sliding" (wording of the spec, not mine) through white codels may be buggy when reaching black/edge codels.

The implementation is sometimes hacky - partly because I'm starting with Rust and am not yet writing ideomatic Rust everywhere, partly because I wanted things to work first and make them beautiful later.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/tessi/rpiet. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

## License

The crate is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the rpiet project’s codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/tessi/rpiet/blob/master/CODE_OF_CONDUCT.md).