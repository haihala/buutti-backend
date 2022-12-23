# Buutti backend task
Recruitment task for Buutti. Backend that listens to port 9000 and can store information on books

Written in the `rust` language using the [rocket](https://rocket.rs/) framework. Using the (diesel)[http://diesel.rs/] ORM to access the `sqlite3` database.

In hindsight, a more tried and tested tools could've whipped up a similar product in half the time. I did learn a thing or do while doing it, so it wasn't time wasted

## Getting started
To start the service (On windows):
1. Install rust through [rustup](https://rustup.rs/)
2. Run `cargo run` in the repo root directory

Only tested on Windows, as I don't currently have access to other options. There may be issues with sqlite. Had to include both the `.lib` and `.dll` files to get it to compile and run. [CI](#ci) runs on Ubuntu, so if you are trying to get it running on linux, that is a good place to start. You probably need to install sqlite dev libraries at least.

## Quality Assurance
Use `cargo test -- --test-threads=1` to run tests. You need the extra flags, since all the tests use the database and cargo parallelizes tests by default. `empty.db` is an empty database with the same schem as `books.db`, which is used to start tests on a clean slate.

`clippy` is used as a linter. `rustfmt` is used as a formatter. `cargo-udeps` is used to check that all dependencies are used. Udeps requires nightly.

### CI
Github actions is configured to do the following:
- Install dependencies
- Build
- Run tests with `cargo test -- --test-threads=1`
- See if `clippy`, `rustfmt`, and `cargo-udeps` are satisfied.
- See if the service starts and stays up for five seconds (this isn't the case with certain missing library configurations)

Cache is used to reduce build and install times.
