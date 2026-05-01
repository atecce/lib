use bible::io;
use bible::kjv;

#[test]
fn word() {
    assert!(kjv::get_word() == io::read_all());
}
