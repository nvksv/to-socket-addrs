//! # to-socket-addrs
//!  
//! A small replacement for `std::net::ToSocketAddrs` for specifying addresses without a port.
//!
//! [![Build Status](https://github.com/nvksv/to-socket-addrs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/nvksv/to-socket-addrs/actions)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![Latest Version](https://img.shields.io/crates/v/to-socket-addrs.svg)](https://crates.io/crates/to-socket-addrs)
//! [![maybe-async](https://docs.rs/to-socket-addrs/badge.svg)](https://docs.rs/to-socket-addrs)
//! 
//! ## Usage
//!
//! To use this crate, add `to-socket-addrs` as a dependency to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! to-socket-addrs = "0.2"
//! ```
//!
//! After that, just use `ToSocketAddrsWithDefaultPort` instead of `ToSocketAddrs` and specify the
//! default port number using `.with_default_port(...)` when creating a stream or listener.
//! 
//! Asynchronous 
//!
//! ## Explanation
//!
//! The standard library assumes explicit indication of the port number when creating a stream or
//! listener:
//!
//! ```rust
//! use std::net::{TcpStream, ToSocketAddrs};
//!
//! fn connect_http<A: ToSocketAddrs>(addr: A) -> TcpStream {
//!     TcpStream::connect(addr).unwrap()
//! }
//!
//! let stream = connect_http("www.google.com:80");
//! ```
//!
//! Most often there is a certain standard port number (80 for HTTP, 21 for FTP, etc), which should
//! be used by default if the user specifies only the server address without explicitly specifying
//! the port number.
//!
//! An ordinary user does not know what the port number is and which one should be specified in each
//! case. The user usually just wants to specify the address (DNS name) of the server.
//!
//! But the naive call `connect_http("www.google.com")` will fail. You should force the user to
//! enter addresses along with the port number. Or you have to process its input and add the port
//! number if there is none (for example, from `"www.google.com"` to `"www.google.com:80"`).
//!
//! It's inconvenient.
//!
//! Therefore you can use this crate and write simply:
//!
//! ```rust
//! use std::net::TcpStream;
//! use to_socket_addrs::ToSocketAddrsWithDefaultPort as ToSocketAddrs;
//!
//! fn connect_http<A: ToSocketAddrs>(addr: A) -> TcpStream {
//!     TcpStream::connect(addr.with_default_port(80)).unwrap()
//! }
//!
//! let stream = connect_http("www.google.com");
//! ```
//!
//! The `.with_default_port(...)` function will check if the port number is specified and add it if
//! necessary.
maybe_async_cfg::content! {

#![maybe_async_cfg::default(
//    disable,
    idents(
        async_std(sync="std", async, tokio="tokio"),
        ToSocketAddrs(use, sync, async="ToSocketAddrsAsync", tokio="ToSocketAddrsTokio"),
        ToSocketAddrsWithDefaultPort(sync, async="ToSocketAddrsWithDefaultPortAsync", tokio="ToSocketAddrsWithDefaultPortTokio"),
        into_vec(fn, tokio="into_vec_tokio"),
    )
)]

use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, IpAddr, Ipv4Addr, Ipv6Addr};

#[maybe_async_cfg::maybe(
    sync(key="sync", feature="sync"),
    async(key="async", feature="async"), 
    async(key="tokio", feature="tokio"), 
)]
use async_std::net::ToSocketAddrs;

/// A trait to use instead of ToSocketAddrs
#[maybe_async_cfg::maybe(
    sync(key="sync", feature="sync"),
    async(key="async", feature="async"), 
    async(key="tokio", feature="tokio"), 
)]
pub trait ToSocketAddrsWithDefaultPort {
    type Inner: Sized + ToSocketAddrs;
    fn with_default_port(&self, default_port: u16) -> Self::Inner;
}


// This types already hold port inside (default port must be ignored)
macro_rules! std_impl {
    ($ty:ty) => {
        #[maybe_async_cfg::maybe(
            keep_self, 
            sync(key="sync", feature="sync"),
            async(key="async", feature="async"), 
            async(key="tokio", feature="tokio"), 
        )]
        impl ToSocketAddrsWithDefaultPort for $ty {
            type Inner = Self;
            fn with_default_port(&self, _default_port: u16) -> Self::Inner {
                *self
            }
        }
    }
}

std_impl!(SocketAddr);
std_impl!(SocketAddrV4);
std_impl!(SocketAddrV6);
std_impl!((IpAddr, u16));
std_impl!((Ipv4Addr, u16));
std_impl!((Ipv6Addr, u16));


// This types hold IP address only, so we always have to use default port
macro_rules! tuple_impl {
    ($ty:ty) => {
        #[maybe_async_cfg::maybe(
            keep_self, 
            sync(key="sync", feature="sync"),
            async(key="async", feature="async"), 
            async(key="tokio", feature="tokio"), 
        )]
        impl ToSocketAddrsWithDefaultPort for $ty {
            type Inner = (Self, u16);
            fn with_default_port(&self, default_port: u16) -> Self::Inner {
                (*self, default_port)
            }
        }
    }
}

tuple_impl!(IpAddr);
tuple_impl!(Ipv4Addr);
tuple_impl!(Ipv6Addr);


#[maybe_async_cfg::maybe(
    sync(key="sync", feature="sync"),
    async(key="async", feature="async"), 
    async(key="tokio", feature="tokio"), 
)]
impl<'s> ToSocketAddrsWithDefaultPort for &'s [SocketAddr] {
    type Inner = &'s [SocketAddr];
    fn with_default_port(&self, _default_port: u16) -> Self::Inner {
        self
    }
}

#[maybe_async_cfg::maybe(
    sync(key="sync", feature="sync"),
    async(key="async", feature="async"), 
    async(key="tokio", feature="tokio"), 
)]
impl<T: ToSocketAddrs + ?Sized> ToSocketAddrsWithDefaultPort for &T where T: ToSocketAddrsWithDefaultPort {
    type Inner = <T as ToSocketAddrsWithDefaultPort>::Inner;
    fn with_default_port(&self, default_port: u16) -> Self::Inner {
        (**self).with_default_port( default_port )
    }
}


macro_rules! str_impl {
    ($ty:ty) => {
        #[maybe_async_cfg::maybe(
            keep_self,
            sync(key="sync", feature="sync"),
            async(key="async", feature="async"), 
            async(key="tokio", feature="tokio"), 
        )]
        impl ToSocketAddrsWithDefaultPort for $ty {
            type Inner = String;

            fn with_default_port(&self, default_port: u16) -> Self::Inner {
                let inner = if let Some(pcolon) = self.rfind(":") {
                    if let Some(pbracket) = self.rfind("]") {
                        if pbracket < pcolon {
                            // "__]__:__" => IPv6 in brackets with port
                            self.to_string()
                        } else {
                            // "__:__]__" => IPv6 in brackets without port
                            format!("{}:{}", self, default_port)
                        }
                    } else {
                        // "__:__", no brackets => IPv4 with port or bare IPv6
                        if let Some(_) = self[..pcolon].rfind(":") {
                            // "__:__:__", no brackets => bare IPv6
                            format!("[{}]:{}", self, default_port)
                        } else {
                            // "__:__", no brackets, no more colons => IPv4 with port
                            self.to_string()
                        }
                    }
                } else {
                    // "__", no colons => IPv4 without port
                    format!("{}:{}", self, default_port)
                };
                inner
            }
        }
    }
}

str_impl!(str);
str_impl!(String);


//#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[maybe_async_cfg::maybe(
        sync(key="sync", feature="sync"),
        async(key="tokio", feature="tokio"), 
        async(key="async", feature="async"), 
    )]
    #[maybe_async_cfg::only_if(sync)]
    fn into_vec<A: ToSocketAddrsWithDefaultPort>(addr: A, default_port: u16) -> Vec<String> {
        let mut v: Vec<String> = addr.with_default_port(default_port).to_socket_addrs().unwrap().map(|a| a.to_string()).collect();
        v.sort();
        v
    }

    #[maybe_async_cfg::maybe(
        sync(key="sync", feature="sync"),
        async(key="async", feature="async"), 
        async(key="tokio", feature="tokio"), 
    )]
    #[maybe_async_cfg::only_if(async)]
    async fn into_vec<A: ToSocketAddrsWithDefaultPort>(addr: A, default_port: u16) -> Vec<String> {
        let mut v: Vec<String> = addr.with_default_port(default_port).to_socket_addrs().await.unwrap().map(|a| a.to_string()).collect();
        v.sort();
        v
    }

    #[maybe_async_cfg::maybe(
        sync(key="sync", feature="sync"),
        async(key="async", feature="async"), 
        async(key="tokio", feature="tokio"), 
    )]
    #[maybe_async_cfg::only_if(tokio)]
    async fn into_vec<A: ToSocketAddrsWithDefaultPort>(addr: A, default_port: u16) -> Vec<String> {
        let mut v: Vec<String> = vec![];
        v.sort();
        v
    }

    #[maybe_async_cfg::maybe(
        sync(key="sync", feature="sync", test), 
        async(key="async", feature="async", async_attributes::test),
        async(feature="tokio", self="ipv_tokio")
    )]
    async fn ipv4() {
        // IPv4 without port
        assert_eq!(into_vec("8.8.8.8", 443).await,            ["8.8.8.8:443"]);
        // IPv4 with port
        assert_eq!(into_vec("8.8.8.8:8080", 443).await,       ["8.8.8.8:8080"]);
    }

    #[maybe_async_cfg::maybe(
        sync(key="sync", feature="sync", test), 
        async(key="async", feature="async", async_attributes::test),
        async(key="tokio", feature="tokio", self="ipv6_tokio", tokio::test)
    )]
    async fn ipv6() {
        // IPv6 without port
        assert_eq!(into_vec("::1", 80).await,                 ["[::1]:80"]);
        assert_eq!(into_vec("[::1]", 80).await,               ["[::1]:80"]);
        assert_eq!(into_vec("[::1]:31337", 80).await,         ["[::1]:31337"]);
    }

    #[maybe_async_cfg::maybe(
        sync(key="sync", feature="sync", test), 
        async(key="async", feature="async", async_attributes::test),
        async(key="tokio", feature="tokio", self="dns_ipv4_tokio", tokio::test)
    )]
    async fn dns_ipv4() {
        // DNS without port (must be resolved to IPv4)
        assert_eq!(into_vec("dns.google", 5353).await,        ["8.8.4.4:5353", "8.8.8.8:5353"]);
        assert_eq!(into_vec("dns.quad9.net", 53).await,       ["149.112.112.112:53", "9.9.9.9:53"]);
        assert_eq!(into_vec("dns11.quad9.net", 3389).await,   ["149.112.112.11:3389", "9.9.9.11:3389"]);
        // DNS with port (must be resolved to IPv4)
        assert_eq!(into_vec("dns.google:53", 8080).await,     ["8.8.4.4:53", "8.8.8.8:53"]);
        assert_eq!(into_vec("dns.quad9.net:80", 53).await,    ["149.112.112.112:80", "9.9.9.9:80"]);
        assert_eq!(into_vec("dns11.quad9.net:21", 3389).await,["149.112.112.11:21", "9.9.9.11:21"]);
    }

    #[maybe_async_cfg::maybe(
        sync(key="sync", feature="sync", test), 
        async(key="async", feature="async", async_attributes::test),
        async(key="tokio", feature="tokio", self="dns_ipv6_tokio", tokio::test)
    )]
    #[cfg_attr(not(feature="test_dns_ipv6"), ignore)]
    async fn dns_ipv6() {
        // DNS without port (must be resolved to IPv6)
        assert_eq!(into_vec("dns64.dns.google", 53).await,        ["[2001:4860:4860::6464]:53", "[2001:4860:4860::64]:53"]);
        // DNS with port (must be resolved to IPv6)
        assert_eq!(into_vec("dns64.dns.google:443", 53).await,    ["[2001:4860:4860::6464]:443", "[2001:4860:4860::64]:443"]);
    }
}

}
