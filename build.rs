extern crate embed_resource;

fn main() {
    embed_resource::compile("buildDeps/tray-icons.rc", embed_resource::NONE);
}