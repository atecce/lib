use equities::item::Reported;

pub fn assert(mut actual: Vec<Reported>, expected: Vec<Reported>) {
    // The expected statements list money items only; share counts and
    // per-share amounts are checked separately.
    actual.retain(|r| r.item.unit() == equities::item::Unit::Money);
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
