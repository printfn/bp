# bp

`bp` is a cross-platform clipboard tool written in Rust, based on
[cb](https://gist.github.com/RichardBronosky/56d8f614fab2bacdd8b048fb58d0c0c7).
It can automatically detect whether to copy or paste, and is
optimised to work in pipes. Its behaviour is based on the
[`tee`](https://man7.org/linux/man-pages/man1/tee.1.html)
command.

## Installation

[`bp`](https://crates.io/crates/bp) is on
[crates.io](https://crates.io/crates/bp), so you can install
it by running:

```bash
cargo install bp
```

## Usage

### Copy

```bash
# copy some text
echo "some text" | bp

# copy the HTML from `example.com`
curl https://example.com | bp

# copy the contents of `file.txt`
bp file.txt

# you can also pipe in a file
bp <file.txt
```

### Paste

```bash
# paste to standard output
bp

# paste to a pipe
bp | jq | less

# paste to `out.txt`
bp >out.txt
```

### Chaining

```bash
# remove formatting from copied text
bp | bp

# prettify, copy and view `example.json`
cat example.json | jq | bp | less

# edit your current clipboard
bp | vipe | bp

# hex-encode contents of the clipboard
bp | xxd | bp
```

### Strip Whitespace

```bash
# copy some text with whitespace stripped
echo " some text " | bp -s

# paste to standard output with whitespace stripped
bp -s
some text%
```

## Licence

`bp` is available under the `GPL-3.0-or-later`.
