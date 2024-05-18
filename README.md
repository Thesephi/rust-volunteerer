# rust-volunteerer

Ever feel the awkward silence when we need someone in a group to do something,
but then nobody raises a hand?

`rust-volunteerer` comes to the rescue. This little program saves us from those awkward moments. Also it's not just time that we save, it's the mental overhead
for everyone involved. So, the larger the group, the more energy we save.

## usages

```bash
./rust-volunteerer # return a name from the roster who should be the "volunteer"

./rust-volunteerer seed # generate example contents for empty db files

./rust-volunteerer employees # display all names from the list (db)

./rust-volunteerer next [name] # given a name existing in the list (db), return the one right after it; if all fails, return the 1st name from the list
```

## build

Prerequisite: [rustup toolchain](https://rustup.rs/)

```bash
# bad
rustc main.rs -o build/main # this won't even build, so don't try it

# good
cargo build [--release]

# or to run immediately after build
cargo run [--release]
```

## miscellaneous

"Why volunteer*er* but not volunteer"? Asked a grammar police once.

"If people actually volunteer, do we still need to make them volunteer?" - answered the project author.
