extern crate ckb_vm;

use ckb_vm::run;
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_invalid_write() {
    let mut file = File::open("tests/programs/invalidwrite").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run(&buffer, &[b"invalidwrite".to_vec()]);
    assert!(result.is_err());
}

#[test]
pub fn test_invalid_exec() {
    let mut file = File::open("tests/programs/invalidexec").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run(&buffer, &[b"invalidexec".to_vec()]);
    assert!(result.is_err());
}