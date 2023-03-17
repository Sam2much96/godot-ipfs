// Rust GDNative implementation of this Godot tutorial:
// https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html#a-custom-node

use gdnative::api::{ Resource, Script, Texture};
use gdnative::prelude::*;

//ipfs
use ipfs_embed::{Ipfs, IpfsOptions};

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
        let options = IpfsOptions::default();
        let ipfs = Ipfs::new(options).await.unwrap();

        // The CID  of the file you want to Download
        let cid = "QmNoThogc1D7XCzQrjePPxChyGmuohX6LXqDTCLJwTUUfR".to_string();

        // Download the File
        let data = ipfs.get(cid).unwrap();

        //print the downloaded file's contents
        println!("{}", String::from_utf8_lossy(&data));
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
