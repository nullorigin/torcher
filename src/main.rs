pub mod macros;
pub mod meta;
pub mod net;
pub mod util;
pub use net::Port;
use net::{PORT_LIST, PROTO_LIST, v4::NetSet};
use std::{env::current_dir, fs::{read_to_string, write}, path::PathBuf};
pub use util::PathVec;
fn main() {
    println!("Torcher v{}", env!("CARGO_PKG_VERSION"));
    println!("Copyright (c) 2025 Matthew Fay. All rights reserved.");
    println!("License: MIT");
    println!("\n");
    let input_dir = current_dir().unwrap().join("in");
    println!("Importing port information from file: /etc/services");
    for port in PORT_LIST.iter() {
        println!("{}", port.to_string());
    }
    println!("Imported {} port service entries", PORT_LIST.len());
    println!("Importing protocol information from file: /etc/protocols");
    for proto in PROTO_LIST.iter() {
        println!("{}", proto.to_string());
    }
    println!("Imported {} protocol entries", PROTO_LIST.len());
    let output_dir = current_dir().unwrap().join("out");
    // Filter for files only and convert to path buffers in one step
    let paths: Vec<PathBuf> = input_dir
        .read_dir()
        .unwrap()
        .filter_map(|p| p.ok().map(|p| p.path()))
        .filter(|p| p.is_file())
        .collect();

    // Process input files and optimize in a single pass
    let mut ns = NetSet::new();
    for (i, path) in paths.iter().enumerate() {
        println!("Adding input file number {} : {}", i + 1, path.display());
        if let Ok(content) = read_to_string(path) {
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
        if let Err(e) = write(output_path, netset.to_string()) {
            println!("Failed to write ip set to target file: ipset{}: {}", i + 1, e);
        }
    }
}
