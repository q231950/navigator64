use std::time::Duration;

fn main() {

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("Available: {}", p.port_name);
    }
    
    println!("Sending to /dev/ttyUSB0 with 57_600 Bauds!");

    let mut port = serialport::new("/dev/ttyUSB0", 57_600)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    let output = "x".as_bytes();
    port.write(output).expect("Write failed!");


}
