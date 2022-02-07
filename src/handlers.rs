use std::convert::Infallible;

use warp::{self, http::StatusCode};
use crate::StateMutex;
use crate::models::stepsPerMl;

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
    state.mode = 0;
    Ok(StatusCode::OK)
}

pub async fn update_status(
    ml_time: (f64, f64),
    state: StateMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let (ml, time) = ml_time;
    state.ml = ml as f64;
    state.steps = (ml * stepsPerMl as f64) as i32;
    state.time = time as f64;
    state.mode = 1;

    Ok(StatusCode::OK)
}
