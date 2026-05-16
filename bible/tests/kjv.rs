#[test]
fn word() {
    assert!(bible::kjv::get_word() == gutenberg::bible::read_all())
}
