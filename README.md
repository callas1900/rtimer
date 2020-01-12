This is a very tiny command line timer with editable progress bar.

# How to use

```bash
# 10 sec timer
$ rtimer 10 
$ rtimer 10 sec

# 10 minutes timer
$ rtimer 10 min

# 1 hour timer
$ rtimer 1 hour

# using finish text 
$ rtimer 1 hour -t finish!
$ rtimer 1 -t finish!

# customize a style of the progress bar.
$ rtimer 10 min -p "=>-"
$ rtimer 10 min -p "01"
```
# Installation

put a built binary file under $PATH as below.

```bash
~/.cargo/bin/rtimer
/usr/local/bin/
```

## using cargo
```bash
$ git clone https://github.com/callas1900/rtimer.git
$ cd rtimer
$ cargo install --path . --force
```

# How to build

```bash
cargo build
```
