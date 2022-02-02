use std::sync::Mutex;
use actix_web::{post, get, web::{self}, web::Data, Error, HttpResponse};
use serde::{
  Serialize, Deserialize
};

use crate::MemRaft;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Number {
  pub value: u32,
}

#[post("/add")]
pub async fn install_snapshot(
  raft: Data<Mutex<MemRaft>>,
  db: Data<Arc<MemStore>>,
  request: web::Json<Number>,
) -> Result<HttpResponse, Error> {
  let raft = raft.lock().unwrap();
  // db.write(request.value) // TODO: TODO: Write value to local DB
  // let response = raft.client_write(rpc) // TODO: Implement this
  // Ok(HttpResponse::Ok().json(response)) // TODO: Enable this
}

#[get("/list")]
pub async fn install_snapshot(
  raft: Data<Mutex<MemRaft>>,
  db: Data<Arc<MemStore>>,
  request: web::Json<Number>,
) -> Result<HttpResponse, Error> {
  let raft = raft.lock().unwrap();
  let response = raft.client_read().await;// TODO: Implement this
  let numbers: Vec<u32> = db.read(...) //... TODO: TODO: Write value from the sync local DB
  // Ok(HttpResponse::Ok().json(response)) // TODO: Return numbers as an JSON array
}