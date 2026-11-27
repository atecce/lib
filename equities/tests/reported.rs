use equities::item::Reported;

pub fn assert(mut actual: Vec<Reported>, expected: Vec<Reported>) {
    actual.sort_by_cached_key(|item| (item.date, item.p, item.item));

    assert_eq!(actual.len(), expected.len(), "\nactual:\n{:#?}\n\nexpected:\n{:#?}\n", actual, expected);

    let zipped = actual.into_iter().zip(expected);
    println!("{:#?}", zipped);

    for (actual, expected) in zipped {
        assert!(
            actual == expected,
            "\nactual:\n{:#?}\n\nexpected:\n{:#?}\n",
            actual, expected,
        )
    }
}
