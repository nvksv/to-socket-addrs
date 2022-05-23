# to-socket-addrs
 
A small replacement for `std::net::ToSocketAddrs` for specifying addresses without a port.

## Usage

To use this crate, add `to-socket-addrs` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
to-socket-addrs = "0.1"
```

After that, just use `ToSocketAddrsWithDefaultPort` instead of `ToSocketAddrs` and specify
the default port number using `.with_default_port(...)` when creating a stream or listener.

## Explanation

The standard library assumes explicit indication of the port number when creating a stream or listener:

```rust
use std::net::{TcpStream, ToSocketAddrs};

fn connect_http<A: ToSocketAddrs>(addr: A) -> TcpStream {
    TcpStream::connect( addr ).unwrap()
}

let stream = connect_http("www.google.com:80");
```

Most often there is a certain standard port number (80 for HTTP, 21 for FTP, etc), which should be used by default if the user specifies only the server address without explicitly specifying the port number. 

An ordinary user does not know what the port number is and which one should be specified in each case. The user usually just wants to specify the address (DNS name) of the server.

But the naive call `connect_http("www.google.com")` will fail. You should force the user to enter addresses along with the port number. Or you have to process its input and add the port number if there is none (for example, from `"www.google.com"` to `"www.google.com:80"`).

It's inconvenient.

Therefore you can use this crate and write simply:

```rust
use std::net::TcpStream;
use to_socket_addrs::ToSocketAddrsWithDefaultPort as ToSocketAddrs;

fn connect_http<A: ToSocketAddrs>(addr: A) -> TcpStream {
    TcpStream::connect( addr.with_default_port(80) ).unwrap()
}

let stream = connect_http("www.google.com");
```

The `.with_default_port(...)` function will check if the port number is specified and add it if necessary.