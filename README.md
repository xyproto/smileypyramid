# Smiley Pyramid

Output a pyramid of smiley faces.

```
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

## Installation

Build with [`sakemake`](https://github.com/xyproto/sakemake) or `qmake`.

### Linux

Install with:

    install -Dm755 smileypyramid /usr/bin/smileypyramid

### BSD or macOS

Build with [`sakemake`](https://github.com/xyproto/sakemake) or `qmake`.

Install with:

    install -d /usr/local/bin
    install -m755 smileypyramid /usr/local/bin/smileypyramid

## Requirements

* docopt
  - Project page: [docopt.cpp](https://github.com/docopt/docopt.cpp)
  - Arch Linux package: [`docopt`](https://www.archlinux.org/packages/community/x86_64/docopt/)
  - AUR package: [`docopt.cpp-git`](https://aur.archlinux.org/packages/docopt.cpp-git/)
  - Debian package: [`libdocopt-dev`](https://packages.debian.org/search?keywords=libdocopt-dev)

If `docopt` is not installed in `/usr/include` and `/usr/bin` you might need to either edit the `Makefile` or edit `smileypyramid.pro` and then generate a new makefile with `qmake`.

## General info

* Version: 1.0
* License: GPL2
* Author: Alexander F Rødseth
