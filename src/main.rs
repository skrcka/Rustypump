//use std::io::prelude::*;
//use std::convert::Infallible;
//use std::net::SocketAddr;
//use hyper::{Body, Request, Response, Server};
//use hyper::service::{make_service_fn, service_fn};
use gpio::{GpioOut};
//use std::{thread, time, fs};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};
use warp::Filter;
use std::time::{Instant};
use configparser::ini::Ini;
use local_ip_address::local_ip;

const ENABLEPIN : u16 = 21; // Green
const DIRPIN : u16 = 16; // Blue
const STEPPIN : u16 = 20; // Purple
const PINSLEEP : u64 = 500000; // Purple
// const KEYPIN : u16 = 12; // KEY1

mod routes;
mod handlers;
mod models;

pub type StateMutex = Arc<Mutex<models::State>>;

async fn sleep_interrupt(sp : StateMutex, prev_running : bool) {
    loop {
        let s = sp.lock().await;
        if prev_running != s.running {
            return;
        }
        time::sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    let mut config = Ini::new();
    let _configmap = config.load("/home/skrcka/config.ini").unwrap();

    let mut state = models::State{running: false, mode: 0, pull: false, ml: 0.0, progress: 100, time_rate: 0.0, steps: 0, steps_per_ml: 0, syringe_size: 0.0, ip: local_ip().unwrap().to_string(), pause: false};
    state.steps_per_ml = config.getint("main", "steps_per_ml").unwrap().unwrap() as i32;
    state.syringe_size = config.getfloat("main", "syringe_size").unwrap().unwrap() as f64;

    let statepointer : StateMutex = Arc::new(Mutex::new(state));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "content-type", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "Access-Control-Allow-Origin"])
        .allow_methods(vec!["POST", "GET"]);
    let routes = routes::routes(statepointer.clone(), config.clone()).with(cors);

    tokio::spawn(async move {
        let mut enable_pin = gpio::sysfs::SysFsGpioOutput::open(ENABLEPIN).unwrap();
        let mut dir_pin = gpio::sysfs::SysFsGpioOutput::open(DIRPIN).unwrap(); // False = push
        let mut step_pin = gpio::sysfs::SysFsGpioOutput::open(STEPPIN).unwrap();
        //let mut key_pin = gpio::sysfs::SysFsGpioInput::open(KEYPIN).unwrap();

        enable_pin.set_value(true).expect("could not set enable_pin");
        dir_pin.set_value(false).expect("could not set dir_pin");
        step_pin.set_value(false).expect("could not set step_pin");
        time::sleep(time::Duration::from_nanos(500)).await;

        let mut time: Instant = Instant::now();
        let mut totalsteps: i32 = 0;
        let mut initial_time: f64 = 0.0;
        let sp = statepointer.clone();
        let mut initial = true;
        let mut ns_per_step: u64 = 500_000_000;
        let mut elapsed_ns: u128;
        let mut prev_running: bool = false;

        loop {
            let mut s = sp.lock().await;
            if s.running{
                if !s.pause {
                    if initial {
                        prev_running = true;
                        
                        enable_pin.set_value(false).expect("could not set enable_pin");
                        dir_pin.set_value(s.pull).expect("could not set dir_pin");
                        time::sleep(time::Duration::from_nanos(PINSLEEP)).await;
                    }
                    // ml/time mode
                    if s.mode == 1 {
                        if initial {
                            time = Instant::now();
                            initial_time = s.time_rate;
                            totalsteps = s.steps;
                            initial = false;
                        }
                        let elapsed = time.elapsed();
                        elapsed_ns = elapsed.as_nanos();
                        s.time_rate = initial_time - (elapsed_ns as f64 / 1_000_000_000.0);
                        let ns_speed_calc = (s.time_rate * 1_000_000_000.0) / s.steps as f64 - (PINSLEEP * 2) as f64;
                        ns_per_step = if ns_speed_calc > 0.0  {ns_speed_calc as u64} else {0};
                        if s.steps > 0 {
                            s.steps -= 1;

                            step_pin.set_value(true).expect("could not set step_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;
                            step_pin.set_value(false).expect("could not set step_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;

                            let prog = 1.0 - (s.steps as f64 / totalsteps as f64);
                            s.progress = (prog * 100.0) as i32;
                            s.ml = (totalsteps as f64 / s.steps_per_ml as f64) - (totalsteps as f64 / s.steps_per_ml as f64) * prog;
                        }
                        else{
                            s.running = false;
                            s.time_rate = 0.0;
                        }
                    }
                    // asap mode
                    else if s.mode == 2 {
                        if initial {
                            ns_per_step = 0;
                            totalsteps = s.steps;
                            initial = false;
                        }
                        
                        if s.steps > 0 {
                            s.steps -= 1;

                            step_pin.set_value(true).expect("could not set step_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;
                            step_pin.set_value(false).expect("could not set step_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;

                            let prog = 1.0 - (s.steps as f64 / totalsteps as f64);
                            s.progress = (prog * 100.0) as i32;
                            s.ml = (totalsteps as f64 / s.steps_per_ml as f64) - (totalsteps as f64 / s.steps_per_ml as f64) * prog;
                        }
                        else{
                            s.running = false;
                        }
                    }
                    // rate mode
                    else if s.mode == 3 {
                        if initial {
                            let ns_speed_calc = ( 1_000_000_000.0 / (s.time_rate * s.steps_per_ml as f64) ) - (2 * PINSLEEP) as f64;
                            ns_per_step = if ns_speed_calc > 0.0  {ns_speed_calc as u64} else {0};
                            totalsteps = s.steps;
                            initial = false;
                        }
                        
                        if s.steps > 0 {
                            s.steps -= 1;

                            step_pin.set_value(true).expect("could not set step_pin"); // await maybe
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;
                            step_pin.set_value(false).expect("could not set step_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;

                            let prog = 1.0 - (s.steps as f64 / totalsteps as f64);
                            s.progress = (prog * 100.0) as i32;
                            s.ml = (totalsteps as f64 / s.steps_per_ml as f64) - (totalsteps as f64 / s.steps_per_ml as f64) * prog;
                        }
                        else{
                            s.time_rate = 0.0;
                            s.running = false;
                        }
                    }
                    // calibration mode
                    else if s.mode == 4 {
                        if initial {
                            dir_pin.set_value(false).expect("could not set dir_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;
                            ns_per_step = 0;
                            initial = false;
                        }
                        
                        if s.steps > 0 {
                            s.steps -= 1;

                            step_pin.set_value(true).expect("could not set step_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;
                            step_pin.set_value(false).expect("could not set step_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;

                            let prog = 1.0 - (s.steps as f64 / totalsteps as f64);
                            s.progress = (prog * 100.0) as i32;
                            s.ml = (totalsteps as f64 / s.steps_per_ml as f64) - (totalsteps as f64 / s.steps_per_ml as f64) * prog;
                        }
                        else{
                            s.running = false;
                        }
                    }
                    // manual mode
                    else if s.mode == 5 {
                        if initial {
                            ns_per_step = 0;
                            initial = false;
                        }
                        
                        if s.steps > 0 {
                            s.steps -= 1;

                            step_pin.set_value(true).expect("could not set step_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;
                            step_pin.set_value(false).expect("could not set step_pin");
                            time::sleep(time::Duration::from_nanos(PINSLEEP)).await;

                            let prog = 1.0 - (s.steps as f64 / totalsteps as f64);
                            s.progress = (prog * 100.0) as i32;
                        }
                        else{
                            s.running = false;
                        }
                    }
                    // keep moving mode
                    else if s.mode == 6 {
                        if initial {
                            ns_per_step = 0;
                            initial = false;
                        }
                        step_pin.set_value(true).expect("could not set step_pin");
                        time::sleep(time::Duration::from_nanos(PINSLEEP)).await;
                        step_pin.set_value(false).expect("could not set step_pin");
                        time::sleep(time::Duration::from_nanos(PINSLEEP)).await;
                    }
                    else{
                        println!("Unsupported mode!");
                    }
                }
            }
            else {
                enable_pin.set_value(true).expect("could not set enable_pin");
                initial = true;
                ns_per_step = 500_000_000;
                prev_running = false;
            }
            drop(s);
            if ns_per_step > 0{
                let sleep = time::sleep(Duration::from_nanos(ns_per_step));
                tokio::pin!(sleep);
                loop {
                    tokio::select! {
                        _ = &mut sleep => {
                            break;
                        }
                        _ = sleep_interrupt(sp.clone(), prev_running) => {
                            break;
                        }
                    }
                }
            }
        }
    });

    warp::serve(routes)
        .run(([0, 0, 0, 0], 5000))
        .await;
}
