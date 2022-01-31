use std::convert::Infallible;
use warp::{self, Filter};

use crate::StateMutex;
use crate::handlers;

pub fn routes(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_status(state.clone())
        .or(update_status(state.clone()))
        .or(stop(state.clone()))
        .or(live_status(state.clone()))
}

fn get_status(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::get_status)
}

fn live_status(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("live_status")
        //.and(warp::ws())
        //.and(warp::path::param())
        .and(with_state(state))
        .and_then(handlers::live_status)
}

fn stop(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("stop")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::stop)
}

fn update_status(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("update_status")
        .and(warp::post())
        .and(json_body())
        .and(with_state(state))
        .and_then(handlers::update_status)
}

fn with_state(state: StateMutex) -> impl Filter<Extract = (StateMutex,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn json_body() -> impl Filter<Extract = ((f64, f64),), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
    .and(warp::body::json())
}
