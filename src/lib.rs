mod ipfs {
    use futures::join;
    use ipfs::{make_ipld, IpfsPath, TestTypes, UninitializedIpfs};
    use tokio::task;

    //use gdnative::core_types::Variant;
    use ipfs::Ipfs;
    use ipfs::IpfsOptions;

    // use gdnative::export::ClassBuilder;
    use gdnative::prelude::*;
   // use gdnative::export::Varargs;

    #[allow(non_camel_case_types)]
    #[derive(NativeClass)]
    #[inherit(Node)]
    
    pub struct IPFS;

    pub trait Method<IPFS>{}

    /*
    impl Method<IPFS> for IPFS{
        fn call(&self, this: TInstance<'_, IPFS>, _args: Varargs<'_>) -> Variant{
            this.map(|obj: &IPFS, _| {
                let result = obj.IPFS();
                Variant::new(result)
            }).expect("method call succeeds")
        }
    }
    */
    impl IPFS {
        fn new(_owner: &Node) -> Self {
            godot_print!("Creating IPFS Node");
            IPFS {}
        }

        /* Trait bound for calling method in Godot
        fn call(&self, this: TInstance<'_, IPFS>, _args: Varargs<'_>) -> Variant{
            this.map(|obj: &IPFS, _| {
                let result = IPFS::run();
                Variant::new(result)
            }).expect("method call succeeds") 
            
        }
        */
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

        /* Register Methods to Native Class
        fn register_methods(builder: &ClassBuilder<Self>) {
            builder.method("IPFS", IPFS)
            .done();
        }
        
        fn start(_owner: &Node) -> Variant {
            //<IPFS as AsyncMethod<IPFS>>::new(owner)
           // fn start(owner: &Node) -> Variant {
           Variant::new("Starting".to_string())
        
        }
        */
    }

    #[methods]
    impl IPFS {
        #[method]
        fn testing(&self, #[base] _owner: &Node) -> bool {
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

/*

thread_local! {
    static EXECUTOR: &'static SharedLocalPool = {
        Box::leak(Box::default())
    };
}
#[derive(Default)]
struct SharedLocalPool {
    local_set: LocalSet,
}

impl futures::task::LocalSpawn for SharedLocalPool {
    fn spawn_local_obj(
        &self,
        future: futures::task::LocalFutureObj<'static, ()>,
    ) -> Result<(), futures::task::SpawnError> {
        self.local_set.spawn_local(future);

        Ok(())
    }
}

#[derive(NativeClass)]
#[inherit(Node)]
struct AsyncExecutorDriver {
    runtime: Runtime,
}

impl AsyncExecutorDriver {
    fn new(_base: &Node) -> Self {
        AsyncExecutorDriver {
            runtime: Builder::new_current_thread()
                .enable_io() // optional, depending on your needs
                .enable_time() // optional, depending on your needs
                .build()
                .unwrap(),
        }
    }
}

#[methods]
impl AsyncExecutorDriver {
    #[method]
    fn _process(&self, _delta: f64) {
        EXECUTOR.with(|e| {
            self.runtime
                .block_on(async {
                    e.local_set
                        .run_until(async { tokio::task::spawn_local(async {}).await })
                        .await
                })
                .unwrap()
        })
    }
}

*/

fn init(handle: InitHandle) {
    //gdnative::tasks::register_runtime(&handle);
    //gdnative::tasks::set_executor(EXECUTOR.with(|e| *e));

    handle.add_class::<IPFS>();

    //handle.add_class::<AsyncExecutorDriver>();
}

godot_init!(init);
