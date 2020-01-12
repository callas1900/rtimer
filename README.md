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
```
# Installation

put a built binary file under $PATH as below.

```bash
~/.cargo/bin/rtimer
```

## using cargo
```bash
$ cargo install --path . --force
```

# How to build

```bash
cargo build
```
