# rust-volunteerer

Ever felt the awkward silence when we need someone in a group to do something,
but then nobody raises a hand?

`rust-volunteerer` comes to the rescue. This little program saves us from those awkward moments. Also it's not just time that we save, it's the mental overhead
for everyone involved. So, the larger the group, the more energy we save.

## usages

Heads up: this (overly-simple) program expects 2 `.csv` files as its "database". There's a command that generates the seed database.

```bash
# generate an example "colleagues" db
./rust-volunteerer seed

# populate the roster for a certain amount of weeks from now
./rust-volunteerer populate

# return from the roster name of a colleague
# who should be the "volunteer" for the current week
./rust-volunteerer

# display all names from the "colleagues" db
./rust-volunteerer colleagues

# given a name existing in the "colleagues" db,
# return the one right after it;
# if all fails, return the 1st name from the list
./rust-volunteerer next [name]
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
