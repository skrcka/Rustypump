//use std::io::prelude::*;
//use std::convert::Infallible;
//use std::net::SocketAddr;
//use hyper::{Body, Request, Response, Server};
//use hyper::service::{make_service_fn, service_fn};
use gpio::{GpioOut};
//use std::{thread, time, fs};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, sleep, Duration};
use warp::Filter;
use std::time::{Instant};

const ENABLEPIN : u16 = 21; // Green
const DIRPIN : u16 = 16; // Blue
const STEPPIN : u16 = 20; // Purple
const PINSLEEP : u64 = 500; // Purple

mod routes;
mod handlers;
mod models;

pub type StateMutex = Arc<Mutex<models::State>>;

async fn sleep_interrupt(sp : StateMutex, prevMode : i32) {
    loop {
        let mut s = sp.lock().await;
        if prevMode != s.mode {
            return;
        }
        time::sleep(Duration::from_millis(500));
    }
}

#[tokio::main]
async fn main() {
    let state = models::State{mode: 0, ml: 0.0, progress: 100, time: 0.0, steps: 0};
    let statepointer : StateMutex = Arc::new(Mutex::new(state));
    let sp = statepointer.clone();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "content-type", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
        .allow_methods(vec!["POST", "GET"]);
    let routes = routes::routes(sp).with(cors);
    let stepsPerMl = models::stepsPerMl;

    tokio::spawn(async move {
        let mut enable_pin = gpio::sysfs::SysFsGpioOutput::open(ENABLEPIN).unwrap();
        let mut dir_pin = gpio::sysfs::SysFsGpioOutput::open(DIRPIN).unwrap(); // False = push
        let mut step_pin = gpio::sysfs::SysFsGpioOutput::open(STEPPIN).unwrap();

        enable_pin.set_value(true).expect("could not set enable_pin");
        dir_pin.set_value(false).expect("could not set dir_pin");
        step_pin.set_value(false).expect("could not set step_pin");
        time::sleep(time::Duration::from_nanos(500));

        let mut time: Instant = Instant::now();
        let mut totalsteps: i32 = 0;
        let mut initialTime: f64 = 0.0;
        let sp = statepointer.clone();
        let mut initial = true;
        let mut nsPerStep: u64 = 500_000_000;
        let mut elapsedNs: u128 = 0;
        let mut prevMode: i32 = 0;
        loop {
            let mut s = sp.lock().await;
            if s.mode == 1 {
                prevMode = 1;
                if initial {
                    enable_pin.set_value(false).expect("could not set enable_pin");
                    time::sleep(time::Duration::from_nanos(PINSLEEP));
                    time = Instant::now();
                    initialTime = s.time;
                    totalsteps = s.steps;
                    initial = false;
                    let nsSpeedCalc = ((initialTime * 1_000_000_000.0) / totalsteps as f64 - (PINSLEEP * 2) as f64);
                    nsPerStep = if nsSpeedCalc > 0.0  {nsSpeedCalc as u64} else {0};
                }
                let elapsed = time.elapsed();
                elapsedNs = elapsed.as_nanos();
                s.time = initialTime - (elapsedNs as f64 / 1_000_000_000.0);
                let nsSpeedCalc = ((initialTime * 1_000_000_000.0) / totalsteps as f64 - (PINSLEEP * 2) as f64);
                nsPerStep = if nsSpeedCalc > 0.0  {nsSpeedCalc as u64} else {0};
                if s.steps > 0 {
                    s.steps -= 1;
                    step_pin.set_value(true).expect("could not set step_pin");
                    time::sleep(time::Duration::from_nanos(PINSLEEP));
                    step_pin.set_value(false).expect("could not set step_pin");
                    time::sleep(time::Duration::from_nanos(PINSLEEP));
                    let prog = 1.0 - (s.steps as f64 / totalsteps as f64);
                    s.progress = (prog * 100.0) as i32;
                    s.ml = (totalsteps as f64 / stepsPerMl as f64) - (totalsteps as f64 / stepsPerMl as f64) * prog;
                }
                else{
                    s.mode = 0;
                    s.time = 0.0;
                }
            }
            else {
                enable_pin.set_value(true).expect("could not set enable_pin");
                initial = true;
                nsPerStep = 500_000_000;
                prevMode = 0;
            }
            drop(s);
            let sleep = time::sleep(Duration::from_nanos(nsPerStep));
            tokio::pin!(sleep);
            loop {
                tokio::select! {
                    _ = &mut sleep => {
                        break;
                    }
                    _ = sleep_interrupt(sp.clone(), prevMode) => {
                        break;
                    }
                }
            }
        }
    });

    warp::serve(routes)
        .run(([0, 0, 0, 0], 5000))
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
