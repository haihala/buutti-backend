# Buutti backend task

Recruitment task for Buutti. Backend that listens to port 9000 and can store information on books

Written in the `rust` language using the [rocket](https://rocket.rs/) framework. Using the (diesel)[http://diesel.rs/] ORM to access the `sqlite3` database.

In hindsight, a more tried and tested tools could've whipped up a similar product in half the time. I did learn a thing or do while doing it, so it wasn't time wasted

## Getting started

To start the service:
1. Install rust through [rustup](https://rustup.rs/)
2. Run `cargo run` in the repo root directory

Only tested on Windows, as I don't currently have access to other options. There may be issues with sqlite. Had to include both the `.lib` and `.dll` files to get it to compile and run.
