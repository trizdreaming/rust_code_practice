extern crate rustc_serialize;

use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use rustc_serialize::json;
use std::io;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
//use std::str;

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

fn build_send_packet(mut stream: &TcpStream, name: String, body: String, msg_type: MessageType){
    let packet = Packet {
        message_id: msg_type,
        group_id: 0,
        sender_name: name,
        body: body
    };

    let encoded = json::encode(&packet).unwrap();
    //println!("{:?}", encoded);

    let send_contents = stream.write(&(encoded).into_bytes());
    let result = stream.flush();

    //println!("send length: {:?}", send_contents.unwrap());
    //println!("is_OK: {:?}", result.unwrap());
}

fn send_packet(mut stream: TcpStream, rx: Receiver<i8>, tx: Sender<i8>) {
    // chatting initialize
    println!("Type Your Nick Name");
    let mut name_input: String = String::new();
    let name_len = io::stdin().read_line(&mut name_input).unwrap();
    build_send_packet(&stream, name_input.clone(), String::new(), MessageType::AddClient);

    // chatting loop
    loop{
        //println!("Type Some Sentence For Talking to Your Group");
        let mut input: String = String::new();
        let user_input = io::stdin().read_line(&mut input);

        match user_input {
            Ok(_) => {
                build_send_packet(&stream, name_input.clone(), input, MessageType::Chat);                
                tx.send(0).unwrap();
            },
            Err(e) => { 
                println!("{:?}", e);
                break;
            }
        }

        // if server shutdowned, thread will be terminated.
        // Receiver.recv() is block function. So send '0' after type contents for pass the recv()
        let server_status = rx.recv();
        match server_status {
            Ok(flag) => {
                if flag == -1 {
                    println!("Send Error By Server ShutDown");
                    drop(stream);
                    break;    
                }
            },
            Err(e) => { 
                println!("{:?}", e);
                break;
            }
        }
        
    }
}

fn receive_packet(mut stream: TcpStream, tx: Sender<i8>) {
    loop {
        let mut read_buf : [u8; 128] = [0;128];
        let result = stream.read(&mut read_buf);

        match result {
            Ok(size) => {
                //println!("read!! {:?}", size);
                if size > 0 {
                    let sliced = &read_buf[0 .. size];
                    let json_string = String::from_utf8_lossy(&sliced);
                    //println!("{:?}", json_string );
                    let decoded:Packet = json::decode(&json_string).unwrap();
                    println!("{:?}: {:?}", decoded.sender_name.trim(), decoded.body.trim());
                }
                else {
                    println!("Server ShutDown!");
                    tx.send(-1).unwrap();
                    break;
                }
            }
            Err(e) => {
                println!("{:?}", e);
                tx.send(-1).unwrap();
                break;
            }
        }
    }
}
    
fn main() {
	println!("Group Chatting Client Running...");
	let connection_info = TcpStream::connect("127.0.0.1:9000");
    let (tx, rx): (Sender<i8>, Receiver<i8>) = channel();

	match connection_info {
		Ok(stream) => {
			println!("connected with Chatting Server");

            // Add access point to stream         
            let clone_stream = stream.try_clone().unwrap();
            let clone_tx = tx.clone();

            // send action thread
            let sender = thread::spawn(move || {
                send_packet(stream, rx, clone_tx);
            });

            //read action thread
            let reader = thread::spawn(move || {
                receive_packet(clone_stream, tx)
            });

            //// Safe way!
            // match clone_stream {
            //     Ok(clone_stream) => {
            //         //println!("stream clone ok");
            //         let reader = thread::spawn(move || {
            //             receive_packet(clone_stream)
            //         });
            //     }
            //     Err(e) => { println!("{:?}", e);}
            // }

            // kill with sub thread
            let sender_terminate_result = sender.join();
            match sender_terminate_result {
                Ok(_) => { println!("sender Thread Exit, now Main thread End"); },
                Err(_) => { println!("join process error"); },
            }

            let reader_terminate_result = reader.join();
            match reader_terminate_result {
                Ok(_) => { println!("reader Thread Exit, now Main thread End"); },
                Err(_) => { println!("join process error"); },
            }
		}

        // catch connnection error
		Err(e) => { 
            println!("{:?}", e);
        }
	}
}