extern crate log2hex;
use log2hex::*;

#[test]
fn log2hex_test() {
    assert_eq!(log2hex(0), "b172");
    assert_eq!(log2hex(1), "1721");
    assert_eq!(log2hex(2), "7217");
    assert_eq!(log2hex(3), "217f");
    assert_eq!(log2hex(4), "17f7");
    assert_eq!(log2hex(100), "ae35");
    assert_eq!(log2hex(128), "3e96");
    assert_eq!(log2hex(1024), "85db");
    assert_eq!(log2hex(65536), "2a18");
}
