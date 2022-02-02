
use std::{sync::Mutex, collections::HashSet};
use actix_web::{post, get, web::{self}, web::Data, Error, HttpResponse};
use openraft::{raft};

use crate::{MemRaft, ClientRequest};

#[post("/append-entries")]
pub async fn append_entries(
  raft: Data<Mutex<MemRaft>>,
  request: web::Json<raft::AppendEntriesRequest<ClientRequest>>,
) -> Result<HttpResponse, Error> {
  let raft = raft.lock().unwrap();
  let response = raft.append_entries(request.0).await.unwrap();
  Ok(HttpResponse::Ok().json(response))
}

#[post("/install-snapshot")]
pub async fn install_snapshot(
  raft: Data<Mutex<MemRaft>>,
  request: web::Json<raft::InstallSnapshotRequest>,
) -> Result<HttpResponse, Error> {
  let raft = raft.lock().unwrap();
  let response = raft.install_snapshot(request.0).await.unwrap();
  Ok(HttpResponse::Ok().json(response))
}

#[post("/vote")]
pub async fn vote(
  raft: Data<Mutex<MemRaft>>,
  request: web::Json<raft::VoteRequest>,
) -> Result<HttpResponse, Error> {
  let raft = raft.lock().unwrap();
  let response = raft.vote(request.0).await.unwrap();
  Ok(HttpResponse::Ok().json(response))
}

#[post("/bootstrap")]
pub async fn bootstrap(
  raft: Data<Mutex<MemRaft>>,
) -> Result<HttpResponse, Error> {
  let raft = raft.lock().unwrap();
  let mut members = HashSet::new(); // TODO: Is this endpoint required?
  members.insert(0);
  members.insert(1);
  members.insert(2);
  let response = raft.initialize(members).await.unwrap();
  Ok(HttpResponse::Ok().json(response))
}

#[get("/metrics")]
pub async fn metrics(
  raft: Data<Mutex<MemRaft>>,
) -> Result<HttpResponse, Error> {
  let raft = raft.lock().unwrap();
  let ch = raft.metrics();
  let metrics = ch.borrow();
  Ok(HttpResponse::Ok().json(format!("{:?}", metrics)))
}