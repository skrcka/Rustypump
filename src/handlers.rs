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
    content: (i32, bool, f64, f64),
    state: StateMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let (mode, pull, ml, time_rate) = content;
    state.ml = ml as f64;
    state.steps = (ml * state.steps_per_ml as f64) as i32;
    state.time_rate = time_rate as f64;
    state.pull = pull;
    state.mode = mode;
    state.running = true;

    Ok(StatusCode::OK)
}

pub async fn manual_move(
    content: (i32, i32, bool),
    state: StateMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let (mode, steps, pull) = content;
    state.steps = steps;
    state.pull = pull;
    state.mode = mode;
    state.running = true;

    Ok(StatusCode::OK)
}

pub async fn update_config(
    content: (i32, i32),
    state: StateMutex,
    config: Ini,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let (steps_per_ml, syringe_size) = content;
    state.steps_per_ml = steps_per_ml;
    state.syringe_size = syringe_size;

    config.write("/home/skrcka/config.ini").unwrap();

    Ok(StatusCode::OK)
}
