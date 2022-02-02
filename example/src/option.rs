use clap::Parser;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {

    #[clap(long, env = "http_addr", default_value = "127.0.0.1:8100")]
    pub http_addr: String,

    #[clap(long, env = "cluster_addr", default_value = "127.0.0.1:8200")]
    pub cluster_http_addr: String,

}

impl Opt {
    
  /// Generating node id from node's remote address
  pub fn generate_node_id(&self) -> u64 {
    let mut hasher = Sha256::new();
    hasher.input_str(&self.http_addr);
    let hash_prefix = &hasher.result_str()[..8];
    let mut buf: [u8; 8] = [0; 8];
    buf.copy_from_slice(hash_prefix.as_bytes());
    u64::from_be_bytes(buf)
  }

}