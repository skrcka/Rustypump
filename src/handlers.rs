use std::convert::Infallible;

use warp::{self, http::StatusCode};
use crate::StateMutex;

pub async fn get_status(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let state = state.lock().await;
    Ok(warp::reply::json(&*state))
}

pub async fn stop(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    state.steps = 0;
    Ok(StatusCode::OK)
}

pub async fn update_status(
    ml_time: (f64, f64),
    state: StateMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let (ml, time) = ml_time;
    state.steps = ml as i32;

    Ok(StatusCode::OK)
}
