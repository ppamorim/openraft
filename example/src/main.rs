use std::sync::Arc;
use std::collections::HashMap;
use actix_web::{middleware, web::Data, App, HttpServer};
use openraft::NodeId;
use openraft::raft::Raft;
use openraft::Config;
use openraft::raft::VoteResponse;
use openraft::raft::VoteRequest;
use openraft::raft::InstallSnapshotResponse;
use openraft::raft::InstallSnapshotRequest;
use openraft::raft::AppendEntriesRequest;
use openraft::raft::AppendEntriesResponse;
use openraft::RaftNetwork;
use memstore::MemStore;
use memstore::ClientRequest;
use memstore::ClientResponse;

mod network;
pub mod option;
pub use option::Opt;

pub type MemRaft = Raft<ClientRequest, ClientResponse, RaftRouter, MemStore>;

pub struct RaftRouter {
  nodes: HashMap<NodeId, String>,
  client: reqwest::Client
}

impl RaftRouter {

  pub fn new(opt: Opt, node_id: NodeId) -> Self {
    RaftRouter {
      nodes: HashMap::from([(node_id, opt.http_addr)]), // TODO: Does it make sense?
      client: reqwest::Client::new()
    }
  }

}

// TODO: Please list all the minimum endpoint required to have the raft working.
#[async_trait]
impl RaftNetwork<ClientRequest> for RaftRouter {
    /// Send an AppendEntries RPC to the target Raft node (§5).
    async fn send_append_entries(
      &self, 
      target: NodeId, 
      rpc: AppendEntriesRequest<ClientRequest>
    ) -> Result<AppendEntriesResponse> {
        let addr = self.nodes.get(&target).unwrap();
        let url = format!("http://{}/append-entries", addr);
        let response: AppendEntriesResponse = self.client
            .post(url)
            .json(&rpc)
            .send()
            .await?
            .json::<AppendEntriesResponse>()
            .await?;
        Ok(response)
    }

    /// Send an InstallSnapshot RPC to the target Raft node (§7).
    async fn send_install_snapshot(
      &self, 
      target: NodeId, 
      rpc: InstallSnapshotRequest
    ) -> Result<InstallSnapshotResponse> {
      let addr = self.nodes.get(&target).unwrap();
      let url = format!("http://{}/install-snapshot", addr);
      let resp = self.client
          .post(url)
          .json(&rpc)
          .send()
          .await?
          .json::<InstallSnapshotResponse>()
          .await?;
      Ok(resp)
    }

    /// Send a RequestVote RPC to the target Raft node (§5).
    async fn send_vote(
      &self, 
      target: NodeId, 
      rpc: VoteRequest
    ) -> Result<VoteResponse> {
      let addr = self.nodes.get(&target).unwrap();
      let url = format!("http://{}/vote", addr);
      let resp = self.client
          .post(url)
          .json(&rpc)
          .send()
          .await?
          .json::<VoteResponse>()
          .await?;
      Ok(resp)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  let opt = Opt::parse();
  let node_id: NodeId = opt.generate_node_id();

  let config = Arc::new(Config::build("primary-raft-group".into())
    .validate()
    .expect("failed to build Raft config"));
  let network = Arc::new(RaftRouter::new(opt.clone(), node_id));
  let storage = Arc::new(MemStore::new(node_id));
  let raft = Raft::new(node_id, config, network, storage);
  let raft_mutex = Data::new(Mutex::new(raft));

  HttpServer::new(move || 
    App::new()
      .wrap(middleware::Compress::default())
      .app_data(db_mutex.clone())
      .app_data(raft_mutex.clone())
      .service(network::raft::append_entries)
      .service(network::raft::install_snapshot)
      .service(network::raft::vote)
      .service(network::raft::bootstrap)
      .service(network::raft::metrics)
      // .service(register)
  )
  .bind(opt.http_addr)?
  .run()
  .await
}
