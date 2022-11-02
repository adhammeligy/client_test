use std::{net::UdpSocket, io};
use std::time;
use std::thread;
fn main() {
    let socket  = UdpSocket::bind("127.0.0.1:7879").expect("couldn't bind to address");
    let sock = UdpSocket::bind("127.0.0.1:21544").expect("couldnt bind to address");
    
    let thread_join_handle = thread::spawn(move || {
        send_request(&socket);
    });

    let thread_join_handle2 = thread::spawn(move || {
        agent(&sock);
    });

    let _res = thread_join_handle.join();
    thread_join_handle2.join().unwrap();


}

fn send_request(socket : &UdpSocket){
    loop {
        let duration = time::Duration::from_secs(1);
        socket.set_read_timeout(Some(duration)).unwrap();
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
        
}

fn agent(socket : &UdpSocket){
    let duration = time::Duration::from_secs(1);
    socket.set_read_timeout(Some(duration)).unwrap();
    let mut buf = [0;1000];
    
}