use rocket::http::Status;

use super::TestClient;

#[test]
fn get_nonexistent_book() {
    let client = TestClient::setup_empty();
    client.get_expect_status("/health-check".into(), Status::Ok);
    client.teardown();
}
