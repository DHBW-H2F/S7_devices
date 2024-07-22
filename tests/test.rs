use s7_device::utils;
use std::fs::File;

#[test]
fn test_defs_read() {
    let file = File::open("tests/test_registers.json").unwrap();
    let defs = utils::get_defs_from_json(file).unwrap();
    assert!(defs.len() == 6, "{0}", defs.len());
}
