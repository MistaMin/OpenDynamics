//! Writes the LV2 description files into a bundle folder:
//! `lv2_ttl <folder> <library stem>` gives `manifest.ttl` and `<stem>.ttl`.
use opendynamics::OpenDynamics;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (Some(dir), Some(stem)) = (args.get(1), args.get(2)) else {
        eprintln!("usage: lv2_ttl <folder> <library stem>");
        std::process::exit(2);
    };
    let plugin = OpenDynamics::default();
    if let Err(e) = openplugin::targets::lv2::write_bundle_for_editor(&plugin, std::path::Path::new(dir), stem, Some(opendynamics::make_editor)) {
        eprintln!("{dir}: {e}");
        std::process::exit(1);
    }
}
