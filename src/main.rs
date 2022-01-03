use std::time::Duration;
use serialport::*;
use std::io::{stdin,stdout,Write};

fn main() {

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("Available: {}", p.port_name);
    }

    println!("Sending to /dev/ttyUSB0 with 57_600 Bauds!");

    let mut port = serialport::new("/dev/ttyUSB0", 57_600)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    read_input(&mut *port);

}

fn read_input<T: SerialPort + ?Sized>(port: &mut T) {

    let mut s = String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    let slash: [u8; 8] = [  0b00000001,
    0b00000010,
    0b00000100,
    0b00001000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b10000000
    ];

    let mut output: [u8; 8] = [0,0,0,0,0,0,0,0];

    if s == String::from("a") {
    output = 
        [
        0b_00000000,
        0b_00111000,
        0b_01000100,
        0b_01000100,
        0b_01111100,
        0b_01000100,
        0b_01000100,
        0b_01000100
        ];
    } else if s == String::from("b") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("c") {
    output = 
        [
        0b_00000000,
        0b_00111000,
        0b_01000100,
        0b_01000000,
        0b_01000000,
        0b_01000000,
        0b_01000100,
        0b_00111000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("e") {
    output = 
        [
        0b_00000000,
        0b_01111100,
        0b_01000000,
        0b_01000000,
        0b_01111000,
        0b_01000000,
        0b_01000000,
        0b_01111100
        ];
    } else if s == String::from("f") {
    output = 
        [
        0b_00000000,
        0b_01111100,
        0b_01000000,
        0b_01000000,
        0b_01111000,
        0b_01000000,
        0b_01000000,
        0b_01000000
        ];
    } else if s == String::from("g") {
    output = 
        [
        0b_00000000,
        0b_00111000,
        0b_01000100,
        0b_01000000,
        0b_01011100,
        0b_01000100,
        0b_01000100,
        0b_00111000
        ];
    } else if s == String::from("1") {
    output = 
        [
        0b_00000000,
        0b_00010000,
        0b_00110000,
        0b_00010000,
        0b_00010000,
        0b_00010000,
        0b_00010000,
        0b_00111000
        ];
    } else if s == String::from("2") {
    output = 
        [
        0b_00000000,
        0b_00111000,
        0b_01000100,
        0b_00000100,
        0b_00001000,
        0b_00010000,
        0b_00100000,
        0b_01111100
        ];
    } else if s == String::from("3") {
    output = 
        [
        0b_00000000,
        0b_00111000,
        0b_01000100,
        0b_00000100,
        0b_00011000,
        0b_00000100,
        0b_01000100,
        0b_00111000
        ];
    } else if s == String::from("4") {
    output = 
        [
        0b_00000000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111100,
        0b_00000100,
        0b_00000100,
        0b_00000100
        ];
    } else if s == String::from("5") {
    output = 
        [
        0b_00000000,
        0b_01111100,
        0b_01000000,
        0b_01000000,
        0b_01111000,
        0b_00000100,
        0b_00000100,
        0b_01111000
        ];
    } else if s == String::from("hi") {
    output = 
        [
        0b_00000000,
        0b_10000010,
        0b_10000000,
        0b_10000010,
        0b_11110010,
        0b_10010010,
        0b_10010010,
        0b_00000000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from("d") {
    output = 
        [
        0b_00000000,
        0b_01111000,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01000100,
        0b_01111000
        ];
    } else if s == String::from(":)") {
    output = 
        [
        0b_00000000,
        0b_00000000,
        0b_00100100,
        0b_00000000,
        0b_00000000,
        0b_01000010,
        0b_00111100,
        0b_00000000
        ];
    }

    port.write(&output[0..1]).expect("Write failed!");
    port.write(&output[1..2]).expect("Write failed!");
    port.write(&output[2..3]).expect("Write failed!");
    port.write(&output[3..4]).expect("Write failed!");
    port.write(&output[4..5]).expect("Write failed!");
    port.write(&output[5..6]).expect("Write failed!");
    port.write(&output[6..7]).expect("Write failed!");
    port.write(&output[7..8]).expect("Write failed!");


    read_input(port);
}
