use std::process::Command;

extern crate gdb;
#[macro_use]
extern crate error_chain;
extern crate toml;
extern crate rustc_serialize;

#[macro_use]
mod error;
mod consts;
mod tas;
mod config;

use std::io::BufRead;

use tas::Tas;
use config::Inputs;

fn main() {
    // set gdb path
    if cfg!(windows) {
        ::std::env::set_var("GDB_BINARY", "./gdb.exe");
    }
    let pid = pidof();
    println!("pid: {}", pid);

    let inputs = Inputs::load("Inputs.toml");

    let std = ::std::io::stdin();
    let lock = std.lock();
    println!("Starting parsing...");
    let frames = tas::parse_lines(lock.lines(), &inputs);
    println!("Parsing finished successfully.");

    println!("Creating tas...");
    let mut tas = Tas::new(pid).unwrap();
    println!("TaS created successfully.");
    handle_err!(tas.init());
    println!("TaS initiated successfully.");
    println!("Waiting for 'New Game'...");
    handle_err!(tas.wait_for_new_game());
    println!("New Game detected. Starting TaS execution");
    handle_err!(tas.play(&frames, &inputs));
}

#[cfg(unix)]
fn pidof() -> u32 {
    let output = Command::new("pidof")
        .arg("Refunct-Linux-Shipping")
        .output()
        .expect("Cannot get pid of Refunct");
    let mut s = String::from_utf8(output.stdout).expect("Output of pidof is not utf8");
    assert_eq!(s.pop(), Some('\n'), "could not get pid of Refunct");
    s.parse().expect("Pidof returned non-number")
}
#[cfg(windows)]
fn pidof() -> u32 {
    let output = Command::new("wmic")
        .args(&["process", "where", "Name='Refunct-Win32-Shipping.exe'", "get", "ProcessId"])
        .output()
        .expect("Cannot get pid of Refunct");
    let s = String::from_utf8(output.stdout).expect("Output of pidof is not utf8");
    let mut lines = s.lines();
    assert_eq!(lines.next().map(|s| s.trim()), Some("ProcessId"), "could not get pid of Refunct");
    lines.next().expect("No line containing pid").trim().parse().expect("Pidof returned non-number")
}
