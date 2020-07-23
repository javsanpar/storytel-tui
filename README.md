# storytel-tui &emsp; [![license]][GPL-3.0] [![rust_min_version]][Rust 1.31]

[license]: https://img.shields.io/badge/license-GPL--3.0-blue
[GPL-3.0]: https://www.gnu.org/licenses/gpl-3.0.html
[rust_min_version]: https://img.shields.io/badge/Rust-1.31+-lightgray.svg
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[cargo install documentation]: https://doc.rust-lang.org/cargo/commands/cargo-install.html


**Listen to your Storytel audiobook library using a lightweight Text User Interface!**

---

- [Supported platforms](#supported-platforms)
- [Building](#building)
  - [Requirements](#requirements)
  - [Crate installation and execution](#crate-installation-and-execution)
- [License](#license)

---

## Supported platforms

This program supports the platforms listed below, other platforms have not
been tested and bugs could appear:

- **GNU/Linux x86_64** (`x86_64-unknown-linux-gnu`)

## Building

### Requirements

- Rust 2018 Edition or greater: `rustc 1.31+`.
- MPV library: `libmpv.so`. This is usually provided by the mpv package.
- OpenSSL.

### Crate installation and execution

You can run the crate directly using `cargo run --release`, this will
create a binary in the `target/release` directory inside the repository's root
and execute it.

Otherwise, you can install the crate in your system with `cargo install`. The path where
the binary is installed depends on different conditions listed in the
[cargo install documentation]. The directory where Cargo installs crates should be included
in your `$PATH` if you wish to execute `storytel-tui` more easily.

## License

The source code of this project is licensed under the GNU General Public License v3.0.
