# Smiley Pyramid

Output a pyramid of smiley faces.

```
$ ./smileypyramid 15
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
```

## Installation

Build with [`sakemake`](https://github.com/xyproto/sakemake), `qmake` or `make`.

### Linux

Install with:

    install -Dm755 smileypyramid /usr/bin/smileypyramid

### BSD or macOS

Build with [`sakemake`](https://github.com/xyproto/sakemake), `qmake`, `make` or `gmake`.

Install with:

    install -d /usr/local/bin
    install -m755 smileypyramid /usr/local/bin/smileypyramid

## Requirements

* docopt
  - Project page: https://github.com/docopt/docopt.cpp
  - AUR package: `docopt.cpp-git`
  - Debian package: `libdocopt-dev`

If `docopt` is not installed in `/usr/include` and `/usr/bin` you might need to either edit the `Makefile` or edit `smileypyramid.pro` and then generate a new makefile with `qmake`.
