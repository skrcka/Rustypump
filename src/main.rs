//use std::io::prelude::*;
//use std::convert::Infallible;
//use std::net::SocketAddr;
//use hyper::{Body, Request, Response, Server};
//use hyper::service::{make_service_fn, service_fn};
//use gpio::{GpioOut};
//use std::{thread, time, fs};
use std::sync::Arc;
use tokio::sync::Mutex;

const ENABLEPIN : u16 = 23; // Green
const DIRPIN : u16 = 24; // Blue
const STEPPIN : u16 = 25; // Purple

mod routes;
mod handlers;
mod models;

pub type StateMutex = Arc<Mutex<models::State>>;

#[tokio::main]
async fn main() {
    let state = models::State{steps: 0, time: 0.0};
    let statepointer : StateMutex = Arc::new(Mutex::new(state));
    let customer_routes = routes::routes(statepointer);

    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;


    /*
    thread::spawn(move || {
        let mut steps : i32 = 0;
        let mut enabled = true;
        let mut enable_pin = gpio::sysfs::SysFsGpioOutput::open(ENABLEPIN).unwrap();
        let mut dir_pin = gpio::sysfs::SysFsGpioOutput::open(DIRPIN).unwrap(); // False = push
        let mut step_pin = gpio::sysfs::SysFsGpioOutput::open(STEPPIN).unwrap();
        
        enable_pin.set_value(true).expect("could not set enable_pin");
        dir_pin.set_value(false).expect("could not set dir_pin");
        step_pin.set_value(false).expect("could not set step_pin");
        
        loop {
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
    });
    */
}
