use futures::join;
use ipfs::{make_ipld, IpfsPath, TestTypes, UninitializedIpfs};
use tokio::task;



use anyhow::{anyhow, Error};
use futures::{future::FutureExt, pin_mut, StreamExt};
use ipfs::{Ipfs, Multiaddr};

use ipfs::p2p::SwarmOptions;
use ipfs::IpfsOptions;
use ipfs::MultiaddrWithPeerId;

use std::convert::TryFrom;

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct IPFS {}

impl IPFS {
    fn new(_owner: &Node) -> Self {
        IPFS{}
    }

    
    async fn run() -> Result<(), ipfs::Error> {
    //tracing_subscriber::fmt::init();

    // Initialize the repo and start a daemon
    let opts = IpfsOptions::inmemory_with_generated_keys();
    let (ipfs, fut): (Ipfs<TestTypes>, _) = UninitializedIpfs::new(opts).start().await.unwrap();
    task::spawn(fut);

    // Create a DAG
    let f1 = ipfs.put_dag(make_ipld!("block1"));
    let f2 = ipfs.put_dag(make_ipld!("block2"));
    let (res1, res2) = join!(f1, f2);
    let root = make_ipld!([res1.unwrap(), res2.unwrap()]);
    let cid = ipfs.put_dag(root).await.unwrap();
    let path = IpfsPath::from(cid);

    // Query the DAG
    let path1 = path.sub_path("0").unwrap();
    let path2 = path.sub_path("1").unwrap();
    let f1 = ipfs.get_dag(path1);
    let f2 = ipfs.get_dag(path2);
    let (res1, res2) = join!(f1, f2);
    println!("Received block with contents: {:?}", res1.unwrap());
    println!("Received block with contents: {:?}", res2.unwrap());

    // Exit
    ipfs.exit_daemon().await;
    Ok(())
}

}

#[methods]
impl IPFS {
    #[export]
    fn _ready(&self, _owner: TRef<Node>) {
        IPFS::run();
    }

    #[export]
    fn start(&self, _owner: TRef<Node>) -> bool {
        let fut = IPFS::run();
        let _ = tokio::spawn(async move {
            if let Err(e) = fut.await {
                godot_error!("IPFS error: {}", e);
            }
        });

        true
    }
}
