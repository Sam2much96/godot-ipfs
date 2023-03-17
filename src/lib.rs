// Rust GDNative implementation of this Godot tutorial:
// https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html#a-custom-node

use gdnative::api::{ Resource, Script, Texture};
use gdnative::prelude::*;

//ipfs
use ipfs_embed::{Ipfs, Config, StorageConfig, NetworkConfig};
use ipfs_embed::identity::ed25519::Keypair;
use std::path::PathBuf;
use std::time::Duration;

#[derive(NativeClass)]
#[inherit(Node)]

//#[register_with(Self::register)]

pub struct IPFSNode;

#[methods]
impl IPFSNode {
    fn new(_base: &Node) -> Self {
        IPFSNode
    }

    #[method]
    fn _enter_tree(&self, #[base] _base: &Node) {
        // Initialization of the plugin goes here.
        // Add the new type with a name, a parent type, a script and an icon.
        let script = unsafe { load::<Script>("res://my_button.gdns", "Script").unwrap() };
        let texture = unsafe {
            load::<Texture>("res://making_plugins-custom_node_icon.png", "Texture").unwrap()
        };
        //_base.add_custom_type("MyButton", "Button", script, texture);
    }

    #[method]
    fn _exit_tree(&self, #[base] _base: &Node) {
        // Clean-up of the plugin goes here.
        // Always remember to remove it from the engine when deactivated.
        //_base.remove_custom_type("MyButton");
        todo!()
    }

    #[tokio::main]
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

        let ipfs = Ipfs::new(config).await.unwrap();

        // The CID  of the file you want to Download
        //let cid = "QmNoThogc1D7XCzQrjePPxChyGmuohX6LXqDTCLJwTUUfR".parse().unwrap();

        // Download the File
        //let data = ipfs.get(cid).unwrap();

        //print the downloaded file's contents
        //println!("{}", String::from_utf8_lossy(&data));
        let status = ipfs.peers().swap(10usize, 10usize);
        println!("{:?}", &status);
    }

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

unsafe fn load<T>(path: &str, hint: &str) -> Option<Ref<T, Shared>>
where
    T: GodotObject<Memory = RefCounted> + SubClass<Resource>,
{
    let resource = ResourceLoader::godot_singleton().load(path, hint, false)?;
    let resource = resource.assume_safe().claim();
    resource.cast::<T>()
}

fn init(handle: InitHandle) {
    handle.add_tool_class::<IPFSNode>();
    handle.add_tool_class::<MyButton>();
}

godot_init!(init);
