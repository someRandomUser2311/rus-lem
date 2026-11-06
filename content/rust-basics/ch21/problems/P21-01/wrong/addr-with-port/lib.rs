use std::io;
use std::net::TcpListener;

pub fn bind_local() -> io::Result<(TcpListener, u16)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok((listener, port))
}

pub fn accept_one(listener: &TcpListener) -> io::Result<String> {
    let (_stream, addr) = listener.accept()?;
    Ok(addr.to_string())
}
