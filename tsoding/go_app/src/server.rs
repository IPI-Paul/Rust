use std::{
    net::{TcpListener, TcpStream, IpAddr, Shutdown, SocketAddr}, result, io::{Read, Write}, 
    fmt, thread, sync::{mpsc::{Receiver, channel, Sender}, Arc}, 
    collections::HashMap, time::{SystemTime, Duration}, str, fmt::Write as OtherWrite
};

use getrandom::getrandom;

type Result<T> = result::Result<T, ()>;

const SAFE_MODE: bool = false;
const BAN_LIMIT: Duration = Duration::from_secs(10 * 60);
const MESSAGE_RATE: Duration = Duration::from_secs(1);
const STRIKE_LIMIT: i32 = 10;

struct Sens<T>(T);

impl<T: fmt::Display> fmt::Display for Sens<T> {
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
    authed: bool,
}

fn server(messages: Receiver<Message>, token: String) -> Result<()> {
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
                    eprintln!("ERROR: could not send banned message to {author_addr}: {err}", author_addr = Sens(author_addr), err = Sens(err));
                });
                let _ = author.shutdown(Shutdown::Both).map_err(|err| {
                    eprintln!("ERROR: could not shutdown socket for {author_addr}: {err}", author_addr = Sens(author_addr), err = Sens(err));
                });
            } else {
                println!("INFO: Client {author_addr} connected", author_addr = Sens(author_addr));
                clients.insert(author_addr.clone(), Client {
                    conn: author.clone(),
                    last_message: now,
                    strike_count: 0,
                    authed: false,
                });
                let _ = write!(author.as_ref(), "Token: ").map_err(|err| {
                    eprintln!("ERROR: could not send Token prompt to {}: {}", Sens(author_addr), Sens(err));
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
                            author.strike_count = 0;
                            if msg != token {
                                println!("INFO: Client {author_addr} sent message {msg}", author_addr = Sens(author_addr));
                            } else {
                                println!("INFO: Client {author_addr} sent correct token!", author_addr = Sens(author_addr));
                            }
                            if author.authed {
                                for (addr, client) in clients.iter() {
                                    if *addr != author_addr && client.authed {
                                        let _ =client.conn.as_ref().write(&bytes).map_err(|err| {
                                            eprintln!("ERROR: could not broadcast message to all the clients from {author_addr}: {err}");
                                        });
                                    }
                                }
                            } else {
                                if msg == token {
                                    author.authed = true;
                                    let _ = writeln!(author.conn.as_ref(), "Welcome to the Club buddy!").map_err(|err| {
                                        eprintln!("ERROR: could not send welcome message to {}: {}", Sens(author_addr), Sens(err));
                                    });
                                } else {
                                    println!("INFO: {} failed authorisation!", Sens(author_addr));
                                    let _ = writeln!(author.conn.as_ref(), "Invalid token!").map_err(|err| {
                                        eprintln!("ERROR: could not notify client {} about invalid token: {}", Sens(author_addr), Sens(err));
                                    });
                                    let _ = author.conn.shutdown(Shutdown::Both).map_err(|err| {
                                        eprintln!("ERROR: could not shutdown {}: {}", Sens(author_addr), Sens(err));
                                    });
                                    clients.remove(&author_addr);
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
            let mut bytes = Vec::new();
            for x in &buffer[0..n] {
                if *x >= 32 || *x == 10 {
                    bytes.push(*x);
                }
            }
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
    let mut buffer = [0; 16];
    let _ = getrandom(&mut buffer).map_err(|err| {
        eprintln!("ERROR: could not generate random access token: {err}");
    });
    let mut token = String::new();
    for x in buffer.iter() {
        let _ = write!(&mut token, "{x:02X}");
    }
    println!("Token: {token}");
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("ERROR: could not bind {address}: {err}", err = Sens(err))
    })?;
    println!("INFO: listening to {}", Sens(address));
    let (message_sender, message_receiver) = channel();
    thread::spawn(|| server(message_receiver, token));
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
