use std::{net::UdpSocket, str};

use pcre2_sys::captures;

const REGEX: &str = r"\d{4}([^\d\s]{3,13})[^\r\n]";

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    loop {
        let mut buf = [0u8; 1500];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let buf = &mut buf[..amt];
        let text = String::from_utf8(buf.to_vec()).unwrap();
        let text = &text.trim();

        println!("Received str: {}", text);

        let captures = captures(REGEX, text);
        if captures.len() == 0 {
            println!("No match");
            socket.send_to("No match".as_bytes(), &src)?;
            continue;
        }

        let mut capts = String::from("Matched: ");

        for i in 0..captures.len() {
            if i > 0 {
                capts.push_str(" and ");
            }
            capts.push_str(captures[i].get(1).unwrap());
        }
        println!("{}", capts);
        socket.send_to(capts.as_bytes(), &src)?;
    }
}
