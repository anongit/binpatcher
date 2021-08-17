# binpatcher

[![CI](https://github.com/anongit/binpatcher/workflows/Continuous%20Integration/badge.svg)](https://github.com/anongit/binpatcher/actions)

## Installation

You can download a pre-built binary from the [releases page](https://github.com/anongit/binpatcher/releases).

## Run

Rename `binpatcher-vx.y.z-windows-x86_64.exe` to `binpatcher.exe`.<br>
Create `binpatcher.bat` file with the following contents:

```bat
binpatcher.exe 00FA0000 FF000000 Baldur.exe Baldur.patched.exe
pause
```

After running you should see `Baldur.patched.exe` and `Baldur.patched.log` files in the current directory.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
