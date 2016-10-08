extern crate libc;

use std::env;
use std::net::TcpListener;
use std::os::unix::io::{AsRawFd,FromRawFd};
use std::process::{Command,Stdio};

fn main() {
    // process arguments
    let mut args = env::args();
    args.next().unwrap();
    let command = args.next().unwrap();
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
    let stdin = unsafe { Stdio::from_raw_fd(libc::dup(stream.as_raw_fd())) };
    let stdout = unsafe { Stdio::from_raw_fd(libc::dup(stream.as_raw_fd())) };
    let stderr = unsafe { Stdio::from_raw_fd(libc::dup(stream.as_raw_fd())) };

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
