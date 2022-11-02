use std::{net::UdpSocket, io};
fn main() {
    let socket  = UdpSocket::bind("127.0.0.1:7879").expect("couldn't bind to address");
    loop {
        let mut buf = [0;1000];
        let mut request = String::new();
        io::stdin()
        .read_line(&mut request)
        .expect("Failed to read input");
        socket.send_to(request.as_bytes(), "127.0.0.1:21543").expect("couldn't send data");
        let respone= socket.recv_from(&mut buf);
        match respone {
            Ok((_,_src_addr)) => {
                let reply = String::from_utf8(buf.to_vec()).unwrap();
                println!("recieved from server : {}",reply)
            }
            Err(_) =>()
        }
        } 
        
    // let (_, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
    // println!("Recieved successsfully from {}",src_addr);
    // let reply = String::from("Ack");
    // let reply =reply.as_bytes();
    // socket.send_to(reply, src_addr).expect("couldn't send data");
}
