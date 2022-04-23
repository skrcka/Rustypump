use std::convert::Infallible;

use warp::{self, http::StatusCode};
use crate::StateMutex;
use configparser::ini::Ini;

pub async fn get_status(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let state = state.lock().await;
    Ok(warp::reply::json(&*state))
}

pub async fn live_status(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let state = state.lock().await;
    Ok(warp::reply::json(&*state))
}

pub async fn stop(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    state.running = false;
    Ok(StatusCode::OK)
}

pub async fn update_status(
    content: (i32, bool, f64, i32, f64, i32),
    state: StateMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let (mode, pull, volume, volume_unit, time_rate, time_rate_unit) = content;
    if volume_unit == 0 {
        state.ml = volume;
    }
    else if volume_unit == 1 {
        state.ml = volume / 1000.0;
    }
    else if volume_unit == 2 {
        state.ml = volume / 1_000_000.0;
    }
    state.steps = (state.ml * state.steps_per_ml as f64) as i32;
    if mode == 1 {
        if time_rate_unit == 0 {
            state.time_rate = time_rate;
        }
        else if time_rate_unit == 1 {
            state.time_rate = time_rate * 60.0;
        }
    }
    else if mode == 3 {
        if time_rate_unit == 0 {
            state.time_rate = time_rate;
        }
        else if time_rate_unit == 1 {
            state.time_rate = time_rate / 1000.0;
        }
        else if time_rate_unit == 2 {
            state.time_rate = time_rate / 1_000_000.0;
        }
    }
    state.pull = pull;
    state.mode = mode;
    state.running = true;

    Ok(StatusCode::OK)
}

pub async fn manual_move(
    content: (i32, bool, i32),
    state: StateMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let (mode, pull, steps) = content;
    state.steps = steps;
    state.pull = pull;
    state.mode = mode;
    state.running = true;

    Ok(StatusCode::OK)
}

pub async fn update_config(
    content: (i32, f64, f64),
    state: StateMutex,
    mut config: Ini,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let (steps_per_ml, syringe_size, ml_in_pump) = content;
    state.steps_per_ml = steps_per_ml;
    state.syringe_size = syringe_size;
    state.ml_in_pump = ml_in_pump;
    
    config.set("main", "steps_per_ml", Some(steps_per_ml.to_string()));
    config.set("main", "syringe_size", Some(syringe_size.to_string()));
    config.set("state", "ml_in_pump", Some(ml_in_pump.to_string()));

    config.write("/home/skrcka/config.ini").unwrap();

    Ok(StatusCode::OK)
}

pub async fn pause(
    state: StateMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    state.pause = !state.pause;

    Ok(StatusCode::OK)
}
