use ffi::Event;
use poll::Poll;
use std::{
    io::{self, Read, Result, Write},
    net::TcpStream,
};
mod ffi;
mod poll;

fn main() -> Result<()> {
    let mut poll = Poll::new()?;
    let n_events = 5;

    let mut streams = vec![];
    let addr = "localhost:8080";

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;

        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);

        let mut stream = std::net::TcpStream::connect(addr)?;

        stream.set_nonblocking(true)?;

        stream.write_all(request.as_bytes())?;
        poll.registry()
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);
    }

    let mut handled_events = 0;

    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);

        poll.poll(&mut events, None);

        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }

        handled_events += handled_events(&events, &mut streams)?;
    }

    Ok(())
}

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
    Host: localhost\r\n\
    Connection: close\r\n\
    \r\n"
    )
}
