use std::str::FromStr;

use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

async fn start_chat(chat_name: &str, name: String) -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8000"))
        .connect()
        .await?;
    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();
    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                          let text_parts = text.splitn(3, " ").collect::<Vec<&str>>();
                          if text_parts[0] == chat_name {
                            println!("Message from {}: {}", text_parts[1], text_parts[2]);
                          }
                        }
                    },
                    Some(Err(err)) => return Err(err.into()),
                    None => {
                      println!("The server connection has dropped. Please check and make sure the server is running properly.");
                      return Ok(());
                    },
                }
            }
            res = stdin.next_line() => {
                match res {
                    Ok(None) => return Ok(()),
                    Ok(Some(line)) => {
                      if line.as_str() == ":q" {
                        println!("Chat room successfully exited.");
                        return Ok(())
                      }
                      let message = format!("{chat_name} {name} {line}");
                      ws_stream.send(Message::text(message)).await?;
                    },
                    Err(err) => return Err(err.into()),
                }
            }

        }
    }
}

async fn private_menu(name: String) {
    loop {
        println!("Input the private chat room name you want to join. Chat room names must be one word. Type :q to exit the private chat menu.");
        println!("Type the name below: ");
        let mut buffer = String::new();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let line_str = buffer.as_str().trim();
                match line_str {
                    ":q" => {
                        break;
                    }
                    "g" | "n" | "a" => {
                        println!("This name cannot be used as a chat room name. Please choose another name.");
                        println!();
                    }
                    _ => {
                        if line_str.contains(' ') {
                            println!("The chat name must be one word. Please try again.");
                            println!("");
                        } else {
                            println!("Joined the room named {}.", line_str);
                            match start_chat(line_str, name.clone()).await {
                                Ok(()) => {}
                                Err(e) => {
                                    println!(
                                        "An error occurred while in the private chat room: {}",
                                        e
                                    );
                                }
                            };
                            println!();
                        }
                    }
                };
            }
            Err(e) => {
                println!("An error occurred while parsing your input: {}", e);
            }
        }
        buffer = String::new();
    }
    ()
}

// fn settings(username: String, private: String) -> Vec<String> {
//     let mut new_settings = vec![username, private];
// }

async fn main_menu(name: String) {
    loop {
        println!("Choose one of the options below.");
        println!("  1 - Global Chat");
        println!("  2 - Private Chat");
        // println!("  3 - Settings");
        println!("  4 - Exit");
        println!("Type your option number below: ");
        let mut buffer = String::new();
        let mut username = name.clone();
        let mut private = "no";
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                match buffer.as_str().trim() {
                    "1" => {
                        println!("You are now in global chat! Enter :q at any time to go back to the main menu.");
                        match start_chat("g", username.clone()).await {
                            Ok(()) => {}
                            Err(e) => {
                                println!("An error occurred while in the private chat room: {}", e);
                            }
                        };
                        println!();
                    }
                    "2" => {
                        println!();
                        private_menu(username.clone()).await;
                        println!();
                    }

                    // "3" => {
                    //     println!();
                    //     let new_settings = settings(username.clone(), private.clone());
                    //     println!();
                    // }
                    "4" => {
                        println!("RustChat successfully exited. Come back soon!");
                        break;
                    }
                    _ => {
                        println!("Invalid input. Please make sure you are inputting one of the 4 option numbers.");
                        println!();
                    }
                };
            }
            Err(e) => {
                println!("An error occurred while parsing your input: {}", e);
            }
        }
        buffer = String::new();
    }
    ()
}

fn set_initial_name() -> String {
    let mut buffer = String::new();
    println!("Welcome to RustChat!");
    loop {
        println!("Input your name below (names must be a single word):");
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let name = buffer.trim().to_string();
                if name.contains(' ') {
                    println!("The name must be one word. Please try again.");
                    println!("");
                } else {
                    println!("So your name is {}? What a wonderful name!", name);
                    println!("");
                    return name;
                }
            }
            Err(e) => {
                println!("An error occurred while parsing your input: {}", e);
            }
        }
        buffer = String::new();
    }
}

#[tokio::main]
async fn main() {
    let name = set_initial_name();
    main_menu(name).await;
}
