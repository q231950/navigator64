use std::time::Duration;

fn main() {

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("Available: {}", p.port_name);
    }
    
    println!("Sending to /dev/ttyUSB1 with 57_600 Bauds!");

    let mut port = serialport::new("/dev/ttyUSB1", 57_600)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    let output = "r".as_bytes();
    port.write(output).expect("Write failed!");
}
