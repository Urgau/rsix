//! The BSD sockets API requires us to read the `ss_family` field before
//! we can interpret the rest of a `sockaddr` produced by the kernel.
#![allow(unsafe_code)]

use super::{SocketAddr, SocketAddrUnix};
use crate::{as_ptr, io};
use linux_raw_sys::general::{__kernel_sockaddr_storage, sockaddr};
use std::mem::size_of;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

// This must match the header of `sockaddr`.
#[repr(C)]
struct sockaddr_header {
    ss_family: u16,
}

/// Read the `ss_family` field from a socket address returned from the OS.
///
/// # Safety
///
/// `storage` must point to a valid socket address returned from the OS.
#[inline]
unsafe fn read_ss_family(storage: *const sockaddr) -> u16 {
    // Assert that we know the layout of `sockaddr`.
    let _ = sockaddr {
        __storage: __kernel_sockaddr_storage {
            ss_family: 0_u16,
            __data: [0; 126_usize],
        },
    };

    (*storage.cast::<sockaddr_header>()).ss_family
}

/// Read a socket address encoded in a platform-specific format.
///
/// # Safety
///
/// `storage` must point to valid socket address storage.
pub(crate) unsafe fn read_sockaddr(storage: *const sockaddr, len: usize) -> io::Result<SocketAddr> {
    let offsetof_sun_path = super::offsetof_sun_path();

    if len < size_of::<linux_raw_sys::general::__kernel_sa_family_t>() {
        return Err(io::Error::INVAL);
    }
    match read_ss_family(storage).into() {
        linux_raw_sys::general::AF_INET => {
            if len < size_of::<linux_raw_sys::general::sockaddr_in>() {
                return Err(io::Error::INVAL);
            }
            let decode = *storage.cast::<linux_raw_sys::general::sockaddr_in>();
            Ok(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::from(u32::from_be(decode.sin_addr.s_addr)),
                u16::from_be(decode.sin_port),
            )))
        }
        linux_raw_sys::general::AF_INET6 => {
            if len < size_of::<linux_raw_sys::general::sockaddr_in6>() {
                return Err(io::Error::INVAL);
            }
            let decode = *storage.cast::<linux_raw_sys::general::sockaddr_in6>();
            Ok(SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::from(decode.sin6_addr.in6_u.u6_addr8),
                u16::from_be(decode.sin6_port),
                u32::from_be(decode.sin6_flowinfo),
                decode.sin6_scope_id,
            )))
        }
        linux_raw_sys::general::AF_UNIX => {
            if len < offsetof_sun_path {
                return Err(io::Error::INVAL);
            }
            if len == offsetof_sun_path {
                Ok(SocketAddr::Unix(SocketAddrUnix::new(&[][..])?))
            } else {
                let decode = *storage.cast::<linux_raw_sys::general::sockaddr_un>();
                assert_eq!(
                    decode.sun_path[len - 1 - offsetof_sun_path],
                    b'\0' as std::os::raw::c_char
                );
                Ok(SocketAddr::Unix(SocketAddrUnix::new(
                    decode.sun_path[..len - 1 - offsetof_sun_path]
                        .iter()
                        .map(|c| *c as u8)
                        .collect::<Vec<u8>>(),
                )?))
            }
        }
        _ => Err(io::Error::NOTSUP),
    }
}

/// Read a socket address returned from the OS.
///
/// # Safety
///
/// `storage` must point to a valid socket address returned from the OS.
pub(crate) unsafe fn read_sockaddr_os(storage: *const sockaddr, len: usize) -> SocketAddr {
    let z = linux_raw_sys::general::sockaddr_un {
        sun_family: 0_u16,
        sun_path: [0; 108],
    };
    let offsetof_sun_path = (as_ptr(&z.sun_path) as usize) - (as_ptr(&z) as usize);

    assert!(len >= size_of::<linux_raw_sys::general::__kernel_sa_family_t>());
    match read_ss_family(storage).into() {
        linux_raw_sys::general::AF_INET => {
            assert!(len >= size_of::<linux_raw_sys::general::sockaddr_in>());
            let decode = *storage.cast::<linux_raw_sys::general::sockaddr_in>();
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::from(u32::from_be(decode.sin_addr.s_addr)),
                u16::from_be(decode.sin_port),
            ))
        }
        linux_raw_sys::general::AF_INET6 => {
            assert!(len >= size_of::<linux_raw_sys::general::sockaddr_in6>());
            let decode = *storage.cast::<linux_raw_sys::general::sockaddr_in6>();
            SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::from(decode.sin6_addr.in6_u.u6_addr8),
                u16::from_be(decode.sin6_port),
                u32::from_be(decode.sin6_flowinfo),
                decode.sin6_scope_id,
            ))
        }
        linux_raw_sys::general::AF_UNIX => {
            assert!(len >= offsetof_sun_path);
            if len == offsetof_sun_path {
                SocketAddr::Unix(SocketAddrUnix::new(&[][..]).unwrap())
            } else {
                let decode = *storage.cast::<linux_raw_sys::general::sockaddr_un>();
                assert_eq!(
                    decode.sun_path[len - 1 - offsetof_sun_path],
                    b'\0' as std::os::raw::c_char
                );
                SocketAddr::Unix(
                    SocketAddrUnix::new(
                        decode.sun_path[..len - 1 - offsetof_sun_path]
                            .iter()
                            .map(|c| *c as u8)
                            .collect::<Vec<u8>>(),
                    )
                    .unwrap(),
                )
            }
        }
        other => unimplemented!("{:?}", other),
    }
}
