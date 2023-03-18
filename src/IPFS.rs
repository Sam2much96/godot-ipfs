// Rust GDNative implementation of this Godot tutorial:
// https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html#a-custom-node

use gdnative::api::{ Resource, Script, Texture};
use gdnative::api::Engine;
use gdnative::prelude::*;
use gdnative::tasks::{Async, AsyncMethod, Spawner};

//ipfs
use ipfs_embed::{Ipfs, Config, StorageConfig, NetworkConfig};
use ipfs_embed::identity::ed25519::Keypair;

use libipld::store::StoreParams;
use libipld::cid::multihash::MultihashDigest;
use libipld::codec::Codec;
use ipfs_embed::Cid;

use std::path::PathBuf;
use std::time::Duration;



#[derive(NativeClass)]
#[inherit(Node)]
//#[feature(inherent_associated_types)]

//#[register_with(Self::register)]

pub struct IPFS<'P>{
    //S : dyn 10usize,
   // P : dyn StoreParams <Hashes = dyn MultihashDigest<10usize>, Codecs = dyn Codec>,
}



trait NativeClassMethods {
        fn new() -> VariantArray{todo!()}
    }
trait NativeClass{
    

        fn new() -> VariantArray{todo!()}
    }

trait usize<'a>{}

impl dyn MultihashDigest<{S}>{
     //type S ; //= dyn usize<'a>;

}

//#[feature(inherent_associated_types)]
#[methods]
impl <P> IPFS<'_>{
    //P : dyn StoreParams <Hashes = dyn MultihashDigest<10usize>, Codecs = dyn Codec>,
    
    
    
    //type S = dyn usize<'a>;
    type P = dyn StoreParams <Hashes = dyn MultihashDigest<{S}>, Codecs = dyn Codec>; // <Hashes = dyn MultihashDigest<10usize>, Codecs = dyn Codec>;
    

    fn new(_base: &Node) -> Self {
        //IPFSNode
        todo!()
    }

    #[tokio::main]
    #[method]
    async fn main() {
        //let config = IpfsOptions::default();
        let path1 = PathBuf::from(r"res://addons/godot-ipfs");
        let path2 = PathBuf::from(r"res://addons/godot-ipfs");
        let path3 = PathBuf::from(r"res://addons/godot-ipfs");
        let five_seconds = Duration::new(5, 0);
        let six_seconds = Duration::new(6, 0);

        let keypair = Keypair::generate();



        let storage_params = StorageConfig{
            path: Some (path1) ,
            access_db_path : Some (path2) ,
            cache_size_blocks : 0u64,
            cache_size_bytes : 0u64,
            gc_interval : five_seconds,
            gc_min_blocks : 0usize,
            gc_target_duration : six_seconds,

        };

        let network_params = NetworkConfig{
            enable_loopback : true,
            port_reuse : true,
            node_name : String::from("IpfsNode"),
            node_key: keypair,
            psk: None,
            dns: None,
            mdns: None,
            kad: None,
            ping: None,
            identify: None,
            gossipsub: None,
            broadcast: None,
            bitswap: None,
            keep_alive: true,


        };

        let config = Config {
            storage: storage_params,
            network: network_params,
        };

        let ipfs : Ipfs<P> = Ipfs::new(config).await.unwrap();

        
        // The CID  of the file you want to Download
        let cid_string = "QmNoThogc1D7XCzQrjePPxChyGmuohX6LXqDTCLJwTUUfR".to_string();
        let cid  = Cid::from_str(cid_string).unwrap();

        // Download the File
        //let data = ipfs.get(cid).unwrap();

        //print the downloaded file's contents
        //println!("{}", String::from_utf8_lossy(&data));
        //let status = ipfs.peers().swap(10usize, 10usize);
        //println!("{:?}", &status);
    }

    //trait NativeClassMethods{
    //    fn 
    //}

    //impl NativeClassMethods for IPFSNode{

    //}



}

#[derive(NativeClass)]
#[inherit(Button)]
struct MyButton;

#[methods]
impl MyButton {
    fn new(_owner: TRef<Button>) -> Self {
        MyButton
    }

    #[method]
    fn _enter_tree(&self, #[base] owner: TRef<Button>) {
        owner
            .connect("pressed", owner, "clicked", VariantArray::new_shared(), 0)
            .unwrap();
    }

    #[method]
    fn clicked(&self) {
        godot_print!("You clicked me!");
    }
}

//unsafe fn load<T>(path: &str, hint: &str) -> Option<Ref<T, Shared>>
//where
//    T: GodotObject<Memory = RefCounted> + SubClass<Resource>,
//{
//    let resource = ResourceLoader::godot_singleton().load(path, hint, false)?;
//    let resource = resource.assume_safe().claim();
//    resource.cast::<T>()
//}

//fn init(handle: InitHandle) {
//    handle.add_tool_class::<IPFS>();
    //handle.add_tool_class::<MyButton>();
//}

//godot_init!(init);
