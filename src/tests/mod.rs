use rocket::local::blocking::Client;
use std::fs::{copy, remove_file, rename};

use super::*;

mod happy_case;

const ACTIVE_DATABASE: &'static str = "books.db";
const BACKUP_DATABASE: &'static str = "books.backup.db";
const EMPTY_DATABASE: &'static str = "empty.db";

fn setup() -> Client {
    rename(ACTIVE_DATABASE, BACKUP_DATABASE).expect("Backup production database");
    copy(EMPTY_DATABASE, ACTIVE_DATABASE).expect("Copy empty database");
    Client::tracked(rocket()).expect("Valid rocket instance")
}

fn teardown(client: Client) {
    drop(client);
    remove_file(ACTIVE_DATABASE).expect("Delete test database after use");
    rename(BACKUP_DATABASE, ACTIVE_DATABASE).expect("Replace active database");
}
