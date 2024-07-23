use s7_device::{types::RegisterValue, utils, S7Connexion, S7Device};
use std::fs::File;
use tokio;

#[test]
fn test_defs_read() {
    let file = File::open("tests/test_registers.json").unwrap();
    let defs = utils::get_defs_from_json(file).unwrap();
    assert!(defs.len() == 6, "{0}", defs.len());
}

fn create_dev() -> S7Device {
    let file = File::open("tests/test_registers.json").unwrap();
    let defs = utils::get_defs_from_json(file).unwrap();
    S7Device::new("127.0.0.1:1102".parse().unwrap(), defs)
}

#[tokio::test]
async fn read_register() {
    let mut dev = create_dev();
    dev.connect().await.unwrap();
    let res = dev
        .read_register_by_name("Test1".to_string())
        .await
        .unwrap();
    if let RegisterValue::Boolean(val) = res {
        assert!(!val);
    }
    let res = dev
        .read_register_by_name("Test3".to_string())
        .await
        .unwrap();
    if let RegisterValue::Boolean(val) = res {
        assert!(val);
    }
}
#[tokio::test]
async fn dump_registers() {
    let mut dev = create_dev();
    dev.connect().await.unwrap();
    let res = dev.dump_registers().await.unwrap();
    println!("{res:?}");
}
