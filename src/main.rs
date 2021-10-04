use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use gpio::{GpioOut};
use std::{thread, time};

const ENABLEPIN : u16 = 23; // Green
const DIRPIN : u16 = 24; // Blue
const STEPPIN : u16 = 25; // Purple

fn main() {
    let mut steps : i32 = 0;
    let mut enabled = true;
    let mut enable_pin = gpio::sysfs::SysFsGpioOutput::open(ENABLEPIN).unwrap();
    let mut dir_pin = gpio::sysfs::SysFsGpioOutput::open(DIRPIN).unwrap(); // False = push
    let mut step_pin = gpio::sysfs::SysFsGpioOutput::open(STEPPIN).unwrap();
    enable_pin.set_value(true).expect("could not set enable_pin");
    dir_pin.set_value(false).expect("could not set dir_pin");


    /*
    let mut value = false;
    thread::spawn(move || loop {
        step_pin.set_value(value).expect("could not set step_pin");
        thread::sleep(time::Duration::from_millis(1000));
        value = !value;
    });
    */

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);

        if steps > 0 {
            println!("{}", steps);
            if enabled == false {
                enabled = true;
                enable_pin.set_value(false).expect("could not set enable_pin");
                thread::sleep(time::Duration::from_millis(500));
            }
            step_pin.set_value(true).expect("could not set step_pin");
            thread::sleep(time::Duration::from_millis(500));
            step_pin.set_value(false).expect("could not set step_pin");
            thread::sleep(time::Duration::from_millis(500));
            steps -= 1;
        } else {
            if enabled == true {
                enabled = false;
                enable_pin.set_value(true).expect("could not set enable_pin");
                thread::sleep(time::Duration::from_millis(500));
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("src/html/index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}