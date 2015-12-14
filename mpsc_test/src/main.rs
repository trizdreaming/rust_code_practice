use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

struct InnerMessage {
    sender_name: String,
    body: String
}

struct ChatGroup {
    group_id: u64,
    tx: Sender<InnerMessage>,
    rx: Receiver<InnerMessage>
    //group_member: HashMap<u32,Client>,
}

fn main() {
    let (tx, rx): (Sender<InnerMessage>, Receiver<InnerMessage>)= channel();
    let test_group = ChatGroup { group_id: 1, tx: tx, rx: rx };

    // thread::spawn(move|| {
    // 	tx.send(10).unwrap();
    // });
    // println!("{:?}", rx.recv().unwrap());
    //assert_eq!(rx.recv().unwrap(), 10);

    // for i in 0..10 {
    // 	let tx = tx.clone();

    // 	thread::spawn(move|| {
    // 		tx.send(i).unwrap();
    // 	});
    // }

    // for k in 0..10 {
    // 	let j = rx.recv().unwrap();
    // 	//assert!(0<=j && j< 10);
    // 	println!("count {:?}, contents {:?}", k, j);

    // }

    //// string test
    // tx.send("hello".to_string()).unwrap();
    // let test = rx.recv().unwrap();

    //// chatGroup test
    let new_msg = InnerMessage {sender_name: "tester_1".to_string(), body: "abcdefg".to_string()};
    test_group.tx.send(new_msg).unwrap();

    let recv_msg = test_group.rx.recv().unwrap();
    println!("sender: {:?}, body: {:?}", recv_msg.sender_name, recv_msg.body);
}
