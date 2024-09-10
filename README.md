# S7_devices
Rust library to provide high level access to the registers in device using the Simatic S7 protocol.

Registers definition is provided as a JSON (see [test_registers.json](/test_registers.json) for the format used).

It is then possible to access a register by it's name, all address and conversion if handled by the library : 
```rust
let port = server.get_host_port_ipv4(102_u16).await.unwrap();

let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

let file = File::open("tests/test_registers.json").unwrap();
let defs = utils::get_defs_from_json(file).unwrap();
let mut dev = S7Device::new(addr, defs)

dev.connect().await.unwrap();

let res = dev.read_register_by_name("Test1").await.unwrap();
```
