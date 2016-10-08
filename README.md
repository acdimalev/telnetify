`telnetify` is a tool for local development of TELNET applications.

It has two important limitations:

- only runs in Unix-like environments
- requires a `telnet` client to be installed


Synopsis
--------

    telnetify COMMAND [COMMAND ARGUMENTS]


Status
------

[![Build Status](https://travis-ci.org/acdimalev/telnetify.svg?branch=master)](https://travis-ci.org/acdimalev/telnetify)
[![Crate Release](https://img.shields.io/crates/v/telnetify.svg)](https://crates.io/crates/telnetify)


Installation
------------

This project uses Cargo for package management.

http://doc.crates.io/

From a clean checkout, you should be able to build and install `telnetify` with
a single command.

    ~$ cargo install


Exposition
----------

To understand `telnetify`, it's important to know that TELNET is a negotiation
protocol.

https://tools.ietf.org/html/rfc854

This may not be particularly obvious since more modern use of `telnet` (for
example, testing an HTTP or SMTP server) does not actually use the TELNET
protocol!

With that in mind, there are really two ways to go about developing a TELNET
server.

- You could write a long-running daemon that handles client connections and
  uses either processes, threads or a select loop to service each connection.

- You could write an application that speaks TELNET over STDIN and STDOUT to a
  single client, and let a super-server (like `inetd`) spawn the application
  for incoming client connections.

One of the potential downsides of the latter approach is needing an easy way to
hook your application up to a TELNET client for development purposes.

Fortunately, that's exactly what `telnetify` addreses.


Demonstration
-------------

Let's demonstrate with Nyancat!

http://nyancat.dakko.us/

    ~$ git clone https://github.com/klange/nyancat.git
    ~$ cd nyancat
    ~/nyancat$ make
    ~/nyancat$ telnetify src/nyancat -t
