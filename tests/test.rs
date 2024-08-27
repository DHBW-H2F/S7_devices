use s7_device::s7_connexion::S7Connexion;
use s7_device::{types::RegisterValue, utils, S7Device};
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use testcontainers::core::WaitFor;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage};
use tokio;

#[test]
fn test_defs_read() {
    let file = File::open("tests/test_registers.json").unwrap();
    let defs = utils::get_defs_from_json(file).unwrap();
    assert!(defs.len() == 6, "{0}", defs.len());
}

async fn create_dev(server: &ContainerAsync<GenericImage>) -> S7Device {
    let port = server.get_host_port_ipv4(102_u16).await.unwrap();

    let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    let file = File::open("tests/test_registers.json").unwrap();
    let defs = utils::get_defs_from_json(file).unwrap();
    S7Device::new(addr, defs)
}

fn create_server() -> GenericImage {
    let server = GenericImage::new("snap7-test-server", "1")
        .with_exposed_port(102.into())
        .with_wait_for(WaitFor::message_on_stdout("Server started"));

    server
}

async fn start_server(container: GenericImage) -> ContainerAsync<GenericImage> {
    let server = container.start().await;
    match server {
        Ok(val) => Ok(val),
        Err(err) => match &err {
            testcontainers::TestcontainersError::Client(client_err) => match client_err {
                testcontainers::core::error::ClientError::PullImage { descriptor: _, err: _pull_err } => panic!("Could not access docker image. Tests need the test server docker image. See https://github.com/lkzjdnb/S7_devices/blob/master/Testing.md \n {err}"),
                _ => Err(err)
            },
            _ => Err(err),
        },
    }
    .unwrap()
}

#[tokio::test]
async fn read_register() {
    let container = create_server();
    let server = start_server(container).await;
    let mut dev = create_dev(&server).await;

    dev.connect().await.unwrap();
    let res = dev.read_register_by_name("Test1").await.unwrap();
    if let RegisterValue::Boolean(val) = res {
        assert!(!val);
    }
    let res = dev.read_register_by_name("Test3").await.unwrap();
    if let RegisterValue::Boolean(val) = res {
        assert!(!val);
    }
}
#[tokio::test]
async fn dump_registers() {
    let container = create_server();
    let server = start_server(container).await;
    let mut dev = create_dev(&server).await;

    dev.connect().await.unwrap();
    let res = dev.dump_registers().await.unwrap();

    assert!(res.len() == 6);
}

#[tokio::test]
async fn write_register() {
    let container = create_server();
    let server = start_server(container).await;
    let mut dev = create_dev(&server).await;

    dev.connect().await.unwrap();
    dev.write_register_by_name("TestInt16", &RegisterValue::S16(69))
        .await
        .unwrap();
    let res = dev.read_register_by_name("TestInt16").await.unwrap();
    assert!(TryInto::<i16>::try_into(res).unwrap() == 69);
}
