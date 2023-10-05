use std::{
    collections::HashMap,
    io::{self, Read, Write}, borrow::BorrowMut,
};

use mio::{tcp::TcpListener, tcp::TcpStream, Events, Poll, PollOpt, Ready, Token};
fn main() {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(&address.parse().unwrap()).unwrap();
    //for maintaining the token socket pair
    let mut counter: usize = 0;
    let mut sockets: HashMap<Token, TcpStream> = HashMap::new();
    let mut response: HashMap<Token, usize> = HashMap::new();
    let mut requests: HashMap<Token, Vec<u8>> = HashMap::new();
    let mut buffer = [0; 1024]; //1k buffer

    let poll = Poll::new().unwrap();
    //We activate by edge for readable events, not level
    poll.register(&listener, Token(0), Ready::readable(), PollOpt::edge())
        .unwrap();

    //Create event pool with capacity of 1024
    let mut events = Events::with_capacity(1024);
    //event loop
    loop {
        poll.poll(&mut events, None).unwrap();
        //make sure here we pass a reference to a loop, for loop moves ownership
        for event in &events {
            /*there are couple types of events:
            1. The incoming connection is ready to be accepted (Our listener token is registered as 0 in line 8)
            2. Event on connected sockets
                2.1 Readable
                2.2 Writable
            */
            match event.token() {
                //event case 1
                Token(0) => {
                    loop {
                        //listener's accept returns io result
                        match listener.accept() {
                            Ok((socket, _)) => {
                                counter += 1;
                                let token = Token(counter);

                                poll.register(&socket, token, Ready::readable(), PollOpt::edge())
                                    .unwrap();
                                sockets.insert(token, socket);
                            }
                            Err(ref e) => {
                                if e.kind() == io::ErrorKind::WouldBlock {
                                    break;
                                } else {
                                    panic!("Unexpected error: {}", e)
                                }
                            }
                        }
                    }
                    
                }
                token if event.readiness().is_readable() => {
                    loop {
                        let read = sockets.get_mut(&token).unwrap().read(&mut buffer);
                        match read {
                            Ok(0) => {
                                // Successful read of zero bytes means connection is closed
                                sockets.remove(&token);
                                break;
                            }
                            Ok(len) => {
                                let req = requests.get_mut(&token).unwrap();
                                for b in &buffer[0..len] {
                                    req.push(*b);
                                }
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                            Err(e) => panic!("Unexpected error: {}", e),
                        }
                    }

                }
                token if event.readiness().is_writable() => {
                    let n_bytes = response[&token];
                    let message = format!("Received {} bytes of data", n_bytes);
                    sockets
                        .get_mut(&token)
                        .unwrap()
                        .write_all(message.as_bytes())
                        .unwrap();
                    response.remove(&token);
                    sockets.remove(&token);
                }
                _ => unreachable!(),
            }
        }
    }
}
