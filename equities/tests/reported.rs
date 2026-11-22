use equities::item::Reported;

pub fn assert(mut actual: Vec<Reported>, expected: Vec<Reported>) {
    // The expected statements list money items only; share counts and
    // per-share amounts are checked separately.
    // Parts of revenue (automotive, financial services) come from segment
    // sections, and total debt sums several lines: not the statement's own
    // lines, so tests check them on their own.
    use equities::item::Item::{AutomotiveDebt, FinancialServicesDebt, TotalDebt};
    actual.retain(|r| r.item.unit() == equities::item::Unit::Money && r.item.part_of().is_none()
        && ![TotalDebt, AutomotiveDebt, FinancialServicesDebt].contains(&r.item));
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
