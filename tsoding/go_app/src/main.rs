use std::{
    net::{TcpListener, TcpStream, IpAddr, Shutdown, SocketAddr}, result, io::{Read, Write}, 
    fmt, thread, sync::{mpsc::{Receiver, channel, Sender}, Arc}, 
    collections::HashMap, time::{SystemTime, Duration}, str
};

type Result<T> = result::Result<T, ()>;

const SAFE_MODE: bool = false;
const BAN_LIMIT: Duration = Duration::from_secs(10 * 60);
const MESSAGE_RATE: Duration = Duration::from_secs(1);
const STRIKE_LIMIT: i32 = 10;

struct Sensitive<T>(T);

impl<T: fmt::Display> fmt::Display for Sensitive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        if SAFE_MODE {
            writeln!(f, "[REDACTED]")
        } else {
            inner.fmt(f)
        }
    }
}

enum Message {
    ClientConnected{
        author: Arc<TcpStream>,
    },
    ClientDisonnected{
        author_addr: SocketAddr,
    },
    NewMessage{
        author_addr: SocketAddr,
        bytes: Vec<u8>,
    },
}

struct Client {
    conn: Arc<TcpStream>,
    last_message: SystemTime,
    strike_count: i32,
}

fn server(messages: Receiver<Message>) -> Result<()> {
    let mut clients = HashMap::<SocketAddr, Client>::new();
    let mut banned_clients = HashMap::<IpAddr, SystemTime>::new();
    loop {
        let msg = messages.recv().expect("The server receiver is no hung up");
        match msg {
        Message::ClientConnected{author} => {
            let author_addr = author.peer_addr().expect("TODO: cache the peer addr of the connection");
            let mut banned_at = banned_clients.remove(&author_addr.ip());
            let now = SystemTime::now();

            banned_at = banned_at.and_then(|banned_at| {
                let diff = now.duration_since(banned_at).expect("TODO: don't crash if the clock went backwards");
                if diff >= BAN_LIMIT {
                    None
                } else {
                    Some(banned_at)
                }
            });

            if let Some(banned_at) = banned_at {
                let diff = now.duration_since(banned_at).expect("TODO: don't crash if the clock went backwards");
                banned_clients.insert(author_addr.ip().clone(), banned_at);
                let mut author = author.as_ref();
                let secs = (BAN_LIMIT - diff).as_secs_f32();
                println!("INFO: Client {author_addr} tried to connect, but that client is banned for {secs} secs");
                let _ = writeln!(author, "You are banned: {secs} secs left").map_err(|err| {
                    eprintln!("ERROR: could not send banned message to {author_addr}: {err}");
                });
                let _ = author.shutdown(Shutdown::Both).map_err(|err| {
                    eprintln!("ERROR: could not shutdown socket for {author_addr}: {err}");
                });
            } else {
                println!("INFO: Client {author_addr} connected");
                clients.insert(author_addr.clone(), Client {
                    conn: author.clone(),
                    last_message: now,
                    strike_count: 0,
                });
            }
        },
            Message::ClientDisonnected{author_addr} => {
                println!("INFO: Client {author_addr} disconnected");
                clients.remove(&author_addr);
            },
            Message::NewMessage{author_addr, bytes} => {
                if let Some(author) = clients.get_mut(&author_addr) {
                    let now = SystemTime::now();
                    let diff = now.duration_since(author.last_message).expect("TODO: don't crash if the clock went backwards");
                    if diff >= MESSAGE_RATE {
                        if let Ok(text) = str::from_utf8(&bytes) {
                            let msg = text.trim();
                            println!("INFO: Client {author_addr} sent message {msg}");
                            for (addr, client) in clients.iter() {
                                if *addr != author_addr {
                                    let _ =client.conn.as_ref().write(&bytes).map_err(|err| {
                                        eprintln!("ERROR: could not broadcast message to all the clients from {author_addr}: {err}");
                                    });
                                }
                            }
                        } else {
                            author.strike_count += 1;
                            if author.strike_count >= STRIKE_LIMIT {
                                println!("INFO: Client {author_addr} got banned");
                                banned_clients.insert(author_addr.ip().clone(), now);
                                let _ = writeln!(author.conn.as_ref(), "You are banned!").map_err(|err| {
                                    eprintln!("ERROR: could not send banned message to {author_addr}: {err}");
                                });
                                let _ = author.conn.shutdown(Shutdown::Both).map_err(|err| {
                                    eprintln!("ERROR: could not shutdown socket for {author_addr}: {err}");
                                });
                            }
                        }
                    } else {
                        author.strike_count += 1;
                        if author.strike_count >= STRIKE_LIMIT {
                            println!("INFO: Client {author_addr} got banned");
                            banned_clients.insert(author_addr.ip().clone(), now);
                            let _ = writeln!(author.conn.as_ref(), "You are banned!").map_err(|err| {
                                eprintln!("ERROR: could not send banned message to {author_addr}: {err}");
                            });
                            let _ = author.conn.shutdown(Shutdown::Both).map_err(|err| {
                                eprintln!("ERROR: could not shutdown socket for {author_addr}: {err}");
                            });
                        }
                    }
                }
            },
        }
    }
}

fn client(stream: Arc<TcpStream>, messages: Sender<Message>) -> Result<()> {
    let author_addr = stream.peer_addr().map_err(|err| {
        eprintln!("ERROR: could not get peer address: {err}");
    })?;
    messages.send(Message::ClientConnected{author: stream.clone()}).map_err(|err| {
        eprintln!("ERROR: could not send message to the server thread: {err}");
    })?;
    let mut buffer = Vec::new();
    buffer.resize(64, 0);
    loop {
        let n = stream.as_ref().read(&mut buffer).map_err(|err| {
            eprint!("ERROR: could not read message from client: {err}");
            let _ = messages.send(Message::ClientDisonnected{author_addr}).map_err(|err| {
                eprintln!("ERROR: could not send message to the server thread: {err}");
            });
        })?;
        if n > 0 {
            let bytes = buffer[0..n].to_vec();
            messages.send(Message::NewMessage{author_addr, bytes}).map_err(|err| {
                eprintln!("ERROR: could not send message to the server thread: {err}");
            })?;
        } else {
            let _ = messages.send(Message::ClientDisonnected{author_addr}).map_err(|err| {
                eprintln!("ERROR: could not send message to the server thread: {err}");
            });
            break;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("ERROR: could not bind {address}: {err}", err = Sensitive(err))
    })?;
    println!("INFO: listening to {}", Sensitive(address));
    let (message_sender, message_receiver) = channel();
    thread::spawn(|| server(message_receiver));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = Arc::new(stream);
                let message_sender = message_sender.clone();
                thread::spawn(|| client(stream, message_sender));
            },
            Err(err) => {
                eprintln!("ERROR: could not accept connection: {err}");
            },
        }
    }
    Ok(())
}
