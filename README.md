# Smiley Pyramid

Output a pyramid of smiley faces.

```sh
$ ./smileypyramid 18
)
:)
:-)
:):)
:-):)
:-):-)
:-):):)
:):):):)
:-):-):-)
:-):):-):)
:-):):):):)
:):):):):):)
:-):-):):-):)
:):):):):):):)
:-):):-):):-):)
:):):):):):):):)
:-):):):):):):):)
:-):-):-):-):-):-)
```

# Rust version

Build and run with `cargo`:

```sh
cargo run --release 18
```

# C++ version

## Installation

Build with [`cxx`](https://github.com/xyproto/cxx).

### Linux

Install with:

```sh
install -Dm755 smileypyramid /usr/bin/smileypyramid
```

### BSD or macOS

Build with [`cxx`](https://github.com/xyproto/cxx).

Install with:

```src
install -d /usr/local/bin
install -m755 smileypyramid /usr/local/bin/smileypyramid
```

## Requirements

* docopt
  - Project page: [docopt.cpp](https://github.com/docopt/docopt.cpp)
  - Arch Linux package: [`docopt`](https://www.archlinux.org/packages/community/x86_64/docopt/)
  - AUR package: [`docopt.cpp-git`](https://aur.archlinux.org/packages/docopt.cpp-git/)
  - Debian package: [`libdocopt-dev`](https://packages.debian.org/search?keywords=libdocopt-dev)

If `docopt` is not installed in `/usr/include` and `/usr/bin` you might need to edit the `Makefile`.

## Benchmarking

From a quick, unscientific and probably misleading benchmark with `time` and a pyramid of size 500, the GCC/C++17 version is around 30% faster than the Rust version of `smileypyramid`. Please try to replicate the result and create a pull request if you have additional results.

## General info

* Version: 1.0.1
* License: GPL2
* Author: Alexander F. Rødseth
