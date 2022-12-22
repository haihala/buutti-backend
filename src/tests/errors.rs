use rocket::http::Status;

use super::TestClient;

#[test]
fn get_nonexistent_book() {
    let client = TestClient::setup();
    client.check_book_expect_status(1, None, Status::NotFound);
    client.teardown();
}

#[test]
fn delete_nonexistent_book() {
    let client = TestClient::setup();
    client.delete_book_expect_status(1, Status::NotFound);
    client.teardown();
}
