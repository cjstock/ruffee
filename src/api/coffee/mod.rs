use std::i64;

use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use super::AppContext;

#[derive(Serialize, Deserialize)]
pub struct Coffee {
    title: String,
    description: String,
    altitude: i64,
}
