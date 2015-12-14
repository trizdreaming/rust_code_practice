extern crate rustc_serialize;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use rustc_serialize::json;
use std::thread;
//use std::thread::sleep_ms;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
//use std::collections::HashMap;
//use std::sync::{Arc, Mutex};

#[derive(RustcDecodable, RustcEncodable)]
struct Packet {
    message_id: MessageType,
    group_id: u64,
    sender_name: String,
    body: String
}

#[derive(RustcDecodable, RustcEncodable)]
enum MessageType{
    Chat,
    System,
    AddClient
}

struct InnerMessage {
    sender_name: String,
    body: String,
    contents_type: MessageType,
    client: TcpStream
}

// todo: this version use just one group, '0'
struct ChatGroup {
    group_id: u64,
    tx: Sender<InnerMessage>,
    rx: Receiver<InnerMessage>,
    //group_member: Vec<Client>
    group_member: Vec<TcpStream>
    //group_member: HashMap<u32,Client>,
}

impl ChatGroup {
    fn new() -> ChatGroup { 
        let (tx, rx): (Sender<InnerMessage>, Receiver<InnerMessage>)= channel();
        
        ChatGroup { 
            group_id: 0, 
            tx: tx, 
            rx: rx, 
            //group_member: Vec::new() 
            group_member: Vec::new()
        }
    }

    fn add_client(&mut self, new_client: TcpStream) {
        self.group_member.push(new_client);
    }

    fn send_to_all_client(&mut self, sender: String, content_body: String) {
        let packet = Packet {
        message_id: MessageType::Chat,
        group_id: 0,
        sender_name: sender,
        body: content_body
        };

        let encoded = json::encode(&packet).unwrap();
        //println!("{:?}", encoded);


        for mut stream in &self.group_member {
            // todo: stream이 생존하고 있는지 체크 필요!
            stream.write(&(encoded).clone().into_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    fn terminate_client() {
        
    }
}

fn handle_incoming_chat(mut stream: TcpStream, tx: Sender<InnerMessage>) {
    loop{
    	let mut read_buf : [u8; 128] = [0;128];
	    let result = stream.read(&mut read_buf); // ignore here too    

	    match result {
	       Ok(size) => {
	           if size > 0 {
	           		let sliced = &read_buf[0 .. size];
           			let json_string = String::from_utf8_lossy(&sliced);
           			let decoded:Packet = json::decode(&json_string).unwrap();
           			//println!("{:?}", decoded.body);

           			match decoded.message_id {
           				MessageType::Chat => {
                            //println!("Sender: {:?}", decoded.sender_name.trim());
           					//println!("{:?}", decoded.body.trim());
                            let copy_stream = stream.try_clone().unwrap();
                            let inner_msg = InnerMessage{sender_name: decoded.sender_name.trim().to_string(), body: decoded.body.trim().to_string(), contents_type: MessageType::Chat, client: copy_stream};
                            tx.send(inner_msg).unwrap();
           				}
           				MessageType::System => { 
           					println!("System Message");
           				}

                        MessageType::AddClient => {
                            println!("{:?} registed!", decoded.sender_name.trim());
                        }
           			}
	           }
	       }
	       Err(e) => { 
                println!("{:?}", e);
                break;
            }
	    }
    }
}

fn handle_chat_group(mut chat_group: ChatGroup) {
    loop {
        let recv_msg_result = chat_group.rx.recv();

        match recv_msg_result {
            Ok(inner_msg) => {
                //println!("inner_msg test: {:?}", contents.sender_name);
                match inner_msg.contents_type {
                    MessageType::AddClient => {
                        let cloned_stream = inner_msg.client.try_clone().unwrap();
                        //chat_group.group_member.push(inner_msg.client);
                        chat_group.add_client(inner_msg.client);

                        let cloned_tx = chat_group.tx.clone();
                        thread::spawn(move || {
                            handle_incoming_chat(cloned_stream, cloned_tx)
                        });
                    }

                    MessageType::Chat => {
                        chat_group.send_to_all_client(inner_msg.sender_name, inner_msg.body);
                    }

                    MessageType::System => { 
                        println!("System Message");
                    }
                }
            },
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }   
    }
    
}

// fn echo_to_client(mut stream: TcpStream) {
//     let mut count = 0;
//     loop {
//         let message = format!("Health check {}", count);
//         let send_contents = stream.write(&(message).into_bytes());
//         let send_result = stream.flush();

//         println!("send length: {:?}", send_contents.unwrap());
//         println!("is_OK: {:?}", send_result.unwrap());

//         // send blocking
//         sleep_ms(5000);
//         count += 1;
//     }
// }

fn main() {
	println!("listening started, ready to accept");
	let listener = TcpListener::bind("127.0.0.1:9000").unwrap();

    // make init group
    //let client_list: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));
    //let init_group: Arc<Mutex<ChatGroup>> = Arc::new(Mutex::new(ChatGroup::new()));
    let init_group = ChatGroup::new();
    let tx = init_group.tx.clone();
    let manager = thread::spawn(move || {
        handle_chat_group(init_group);
    });

    //test
    // let test = init_group.lock().unwrap();
    // println!("group Id: {:?}",test.group_id);
    // panic!();

    // accept connections and process them, spawning a new thread for each one
	for connection_info in listener.incoming() {
		// stream saperate for sending
	    match connection_info {
	        Ok(stream) => {
                //let clone_stream = stream.try_clone().unwrap();

                //add new group member to init group
                // {
                //     let new_client = Client{send_stream: clone_stream, receive_stream: stream};
                //     //init_group.group_member.push(new_client);
                //     let mut locked_group = init_group.group_member.lock().unwrap();
                //     locked_group.push(new_client);
                // }

                // receiver
	            // thread::spawn(move || {
	            //     handle_client(stream, tx.clone())
	            // });

                //// sender
                // thread::spawn(move || {
                //     echo_to_client(clone_stream)
                // });
                let inner_msg = InnerMessage{sender_name: String::new(), body: String::new(), contents_type: MessageType::AddClient, client: stream};
                tx.send(inner_msg).unwrap();
	        }
	        Err(e) => { println!("Server Error{:?}", e); }
	    }
	}

    let manager_terminate_result = manager.join();
    match manager_terminate_result {
        Ok(_) => { println!("server manager Thread Exit, now Main thread End"); },
        Err(_) => { println!("join process error"); },
    }

	drop(listener);
}