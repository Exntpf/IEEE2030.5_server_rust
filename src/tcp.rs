use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::{Error as IoError, ErrorKind, Result as IoResult};
use std::os::unix::io::FromRawFd;

extern crate libc;

pub fn create_tcp_pair() -> IoResult<(TcpStream, TcpStream)> {
    let mut fds: [libc::c_int; 2] = [0; 2];
    unsafe {
        // one might consider creating a TcpStream from a UNIX socket a hack
        // most socket operations should work the same way, and UnixSocket
        // is too new to be used
        if libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) == 0 {
            Ok((
                TcpStream::from_raw_fd(fds[0]),
                TcpStream::from_raw_fd(fds[1]),
            ))
        } else {
            Err(IoError::last_os_error())
        }
    }
}

pub fn listen<T: ToSocketAddrs>(addr: T) -> IoResult<TcpListener>{
    match TcpListener::bind(addr){
        Ok(a) => Ok(a),
        Err(a) => Err(IoError::new(ErrorKind::AddrNotAvailable, a)),
    }
}

pub fn connect<T: ToSocketAddrs>(addr: T) -> IoResult<TcpStream>{
    match TcpStream::connect(addr){
        Ok(a) => Ok(a),
        Err(a) => Err(IoError::new(ErrorKind::NotConnected, a)),
    }
}
