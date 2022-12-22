# Buutti backend task
Recruitment task for Buutti. Backend that listens to port 9000 and can store information on books

Written in the `rust` language using the [rocket](https://rocket.rs/) framework. Using the (diesel)[http://diesel.rs/] ORM to access the `sqlite3` database.

In hindsight, a more tried and tested tools could've whipped up a similar product in half the time. I did learn a thing or do while doing it, so it wasn't time wasted

## Getting started
To start the service (On windows):
1. Install rust through [rustup](https://rustup.rs/)
2. Run `cargo run` in the repo root directory

Only tested on Windows, as I don't currently have access to other options. There may be issues with sqlite. Had to include both the `.lib` and `.dll` files to get it to compile and run. [CI](#ci) runs on Ubuntu, so if you are trying to get it running on linux, that is a good place to start. You probably need to install sqlite dev libraries and for some reason ubuntu couldn't find the database definition in rocket.toml. Hard to debug through actions, so the solution is a bit of a workaround.

## Quality Assurance
Use `cargo test` to run tests. Lint with clippy. Clippy is installed with `rustup component add clippy` and ran with `cargo clippy`. `empty.db` is an empty database with the same schem as `books.db`, which is used to start tests on a clean slate.

### CI
Github actions is configured to do the following:
- Build
- Run tests with `cargo test`
- See if the service starts and stays up for five seconds (this isn't the case with certain missing library configurations)

