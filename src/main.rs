pub mod macros;
pub mod net;
pub mod util;
use net::v4::NetSet;
use std::{env::current_dir, path::PathBuf};
pub use util::PathIO;
fn main() {
    let input_dir = current_dir().unwrap().join("in");
    let output_dir = current_dir().unwrap().join("out");
    let mut pathio = PathIO::new();
    pathio.set_max_depth(10);
    // Filter for files only and convert to path buffers in one step
    let paths: Vec<PathBuf> = input_dir
        .read_dir()
        .unwrap()
        .filter_map(|p| p.ok().map(|p| p.path()))
        .filter(|p| p.is_file())
        .collect();

    pathio.set_paths(paths.clone()).unwrap();

    // Process input files and optimize in a single pass
    let mut ns = NetSet::new();
    for (i, path) in paths.iter().enumerate() {
        println!("Adding input file number {} : {}", i + 1, path.display());
        if let Ok(content) = pathio.read_from_path(i) {
            if let Ok(mut file_set) = NetSet::from_str(&content) {
                ns.append(&mut file_set);
                ns.sort();
                ns.dedup();
            }
        }
    }
    ns.optimize();
    // Split into chunks of max 65535 entries
    let mut netsets = Vec::new();
    if ns.len() > 65535 {
        while ns.len() > 65535 {
            let (chunk, remainder) = ns.split_at(65535);
            netsets.push(chunk);
            ns = remainder;
        }
    }
    if !ns.is_empty() {
        netsets.push(ns);
    }

    // Write output files
    for (i, netset) in netsets.iter().enumerate() {
        let output_path = output_dir.join(format!("ipset{}", i + 1));
        let file_index = i;

        if pathio.set_path(&output_path, file_index).is_ok() {
            match pathio.write_to_path(file_index, &netset.to_string()) {
                Ok(_) => println!("Ip set written to target file: ipset{}", i + 1),
                Err(_) => println!("Failed to write ip set to target file: ipset{}", i + 1),
            }
        }
    }
}
