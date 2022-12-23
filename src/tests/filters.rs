use crate::types::Book;

use super::TestClient;

#[test]
fn test_year_filters() {
    let client = TestClient::setup_with_books();

    let all_books = client.get_all_books();
    assert_eq!(all_books.len(), 7);
    // If the value cannot be parsed to an integer, it is treated as if it didn't exist
    // This may be against the specifications, but it's how rocket handles things by default
    // This can be fixed, if it is.
    assert_eq!(all_books, client.get_book_list("year=".into()));

    let no_books = client.get_book_list("year=3030".into());
    assert_eq!(no_books, vec![]);

    let books_from_1999 = client.get_book_list("year=1999".into());
    assert_eq!(books_from_1999.len(), 2);

    let books_from_2019 = client.get_book_list("year=2019".into());
    assert_eq!(books_from_2019.len(), 2);

    assert_ne!(books_from_1999, books_from_2019);
    // Book list contains two pairs of books with editions from 1999 and 2019.
    for (old, new) in books_from_1999.into_iter().zip(books_from_2019.into_iter()) {
        assert_eq!(old.title, new.title);
        assert_eq!(old.author, new.author);
        assert_eq!(old.publisher, new.publisher);

        assert_ne!(old.description, new.description);
        assert_eq!(old.year, 1999);
        assert_eq!(new.year, 2019);
    }

    client.teardown();
}

#[test]
fn test_title_author_filters() {
    let client = TestClient::setup_with_books();

    let expected_clean_code = vec![Book {
        // It just happens to go into index 3.
        // This couples the test to the test data in an unfortunate way, could maybe just compare non-id fields
        id: Some(3),
        title: "Clean Code".into(),
        author: "Robert C. Martin".into(),
        year: 2008,
        publisher: Some("Pearson".into()),
        description: Some("Much java, such wow".into()),
    }];

    // The search terms are case sensitive. This wasn't specified, but it may be more convenient if they weren't
    let clean_code_by_author = client.get_book_list("author=Robert%20C.%20Martin".into());
    let clean_code_by_title = client.get_book_list("title=Clean%20Code".into());
    assert_eq!(clean_code_by_author, expected_clean_code);
    assert_eq!(clean_code_by_title, expected_clean_code);

    let books_from_david_thomas = client.get_book_list("author=David%20Thomas".into());
    assert_eq!(books_from_david_thomas.len(), 2);

    let pragmatic_programmer_books = client.get_book_list(
        "title=Pragmatic%20Programmer,%20The:%20Your%20journey%20to%20mastery".into(),
    );
    assert_eq!(pragmatic_programmer_books.len(), 2);

    assert!(books_from_david_thomas
        .iter()
        .all(|book| pragmatic_programmer_books.contains(book)));
    assert!(pragmatic_programmer_books
        .iter()
        .all(|book| books_from_david_thomas.contains(book)));

    client.teardown();
}

#[test]
fn year_and_author_test() {
    let client = TestClient::setup_with_books();

    let books_from_david_thomas_from_1999 =
        client.get_book_list("author=David%20Thomas&year=1999".into());

    // Both of those filters individually should yield two books, together only one book should show up
    assert_eq!(
        books_from_david_thomas_from_1999,
        vec![Book {
            id: Some(7),
            title: "Pragmatic Programmer, The: Your journey to mastery".into(),
            author: "David Thomas".into(),
            year: 1999,
            publisher: Some("Addison-Wesley Professional".into()),
            description: Some("First edition".into()),
        }]
    );
    client.teardown();
}
