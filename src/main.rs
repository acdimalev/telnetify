extern crate libc;

use std::env;
use std::net::TcpListener;
use std::os::unix::io::{AsRawFd,FromRawFd};
use std::process::{Command,Stdio};

fn stdio_dup_from<T: AsRawFd>(x: &T) -> Stdio {
    unsafe { Stdio::from_raw_fd(libc::dup(x.as_raw_fd())) }
}

fn usage() {
    println!("Usage: telnetify COMMAND [COMMAND ARGUMENTS]");
}

fn main() {
    // process arguments
    // skip the first argument -- name of our binary
    // print usage if command is not specified
    let mut args = env::args().skip(1);
    let command = match args.next() {
        Some(x) => x,
        None => return usage(),
    };
    let command_args = args.collect::<Vec<_>>();

    // open tcp socket
    let listener = TcpListener::bind("[::1]:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    // convert port to string for later use
    // dash prefix forces telnet protocol
    let str_port = format!("-{}", port);

    // spawn telnet
    let mut telnet = Command::new("telnet")
    .args(&["::1", "--", &str_port])
    .spawn().unwrap();

    // wait for telnet to connect
    let (stream, _) = listener.accept().unwrap();
    drop(listener);
    let stdin = stdio_dup_from(&stream);
    let stdout = stdio_dup_from(&stream);
    let stderr = stdio_dup_from(&stream);

    // spawn application
    let mut application = Command::new(command)
    .args(&command_args)
    .stdin(stdin)
    .stdout(stdout)
    .stderr(stderr)
    .spawn().unwrap();

    // wait for all processes to terminate
    telnet.wait().unwrap();
    application.wait().unwrap();
}
