mod ipfs {
    use futures::join;
    use ipfs::{make_ipld, IpfsPath, TestTypes, UninitializedIpfs};
    use tokio::task;

    //use gdnative::core_types::Variant;
    use ipfs::Ipfs;
    use ipfs::IpfsOptions;

    // use gdnative::export::ClassBuilder;
    use gdnative::prelude::*;

    #[allow(non_camel_case_types)]
    #[derive(NativeClass)]
    #[inherit(Node)]
    
    pub struct IPFS;

    pub trait Method<IPFS>{}


    impl IPFS {
        fn new(_owner: &Node) -> Self {
            godot_print!("Creating IPFS Node For Dag Test");
            IPFS {}
        }

  
        async fn run() -> Result<(), ipfs::Error> {
            let opts = IpfsOptions::inmemory_with_generated_keys();
            let (ipfs, fut): (Ipfs<TestTypes>, _) = UninitializedIpfs::new(opts).start().await.unwrap();
            task::spawn(fut);

            let f1 = ipfs.put_dag(make_ipld!("block1"));
            let f2 = ipfs.put_dag(make_ipld!("block2"));
            let (res1, res2) = join!(f1, f2);
            let root = make_ipld!([res1.unwrap(), res2.unwrap()]);
            let cid = ipfs.put_dag(root).await.unwrap();
            let path = IpfsPath::from(cid);

            let path1 = path.sub_path("0").unwrap();
            let path2 = path.sub_path("1").unwrap();
            let f1 = ipfs.get_dag(path1);
            let f2 = ipfs.get_dag(path2);
            let (res1, res2) = join!(f1, f2);
            godot_print!("Received block with contents: {:?}", res1.unwrap());
            godot_print!("Received block with contents: {:?}", res2.unwrap());

            ipfs.exit_daemon().await;
            Ok(())
        }

     
    }

    #[methods]
    impl IPFS {
        #[method]
        fn _ready(&self, #[base] _owner: &Node) -> bool {
            let fut = IPFS::run();
            let _ = tokio::spawn(async move {
                if let Err(e) = fut.await {
                    godot_error!("IPFS error: {}", e);
                }
            });

            true
        }
        
    }
}


use gdnative::prelude::*;

//use tokio::runtime;
/*
use tokio::{
    runtime::{Builder, Runtime},
   // task::LocalSet,
};

*/
use crate::ipfs::IPFS;



fn init(handle: InitHandle) {

    handle.add_class::<IPFS>();

}

godot_init!(init);
