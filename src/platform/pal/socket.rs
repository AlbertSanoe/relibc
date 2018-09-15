use super::super::types::*;
use super::super::Pal;
use header::sys_socket::{sockaddr, socklen_t};

pub trait PalSocket: Pal {
    unsafe fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;

    unsafe fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;

    unsafe fn connect(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;

    unsafe fn getpeername(
        socket: c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> c_int;

    unsafe fn getsockname(
        socket: c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> c_int;

    unsafe fn recvfrom(
        socket: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> ssize_t;

    unsafe fn sendto(
        socket: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        dest_len: socklen_t,
    ) -> ssize_t;

    unsafe fn socket(domain: c_int, kind: c_int, protocol: c_int) -> c_int;
}