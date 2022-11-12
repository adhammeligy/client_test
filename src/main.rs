use std::{net::UdpSocket, io};
use std::time;
use std::thread;
fn main() {
    let socket  = UdpSocket::bind("10.40.33.121:7877").expect("couldn't bind to address");
    let sock = UdpSocket::bind("10.40.33.121:7878").expect("couldnt bind to address");
    
    let thread_join_handle = thread::spawn(move || {
        generate_request(&socket);
    });

    let thread_join_handle2 = thread::spawn(move || {
        agent(&sock);
    });

    let _res = thread_join_handle.join();
    thread_join_handle2.join().unwrap();


}

fn generate_request(socket : &UdpSocket){
    loop {
        let duration = time::Duration::from_secs(1);
        socket.set_read_timeout(Some(duration)).unwrap();
        let mut buf = [0;1000];
        let mut request = String::new();
        io::stdin()
        .read_line(&mut request)
        .expect("Failed to read input");
        // send to agent the request 
        socket.send_to(request.as_bytes(), "10.40.33.121:7878").expect("couldn't send data"); 
        let respone= socket.recv_from(&mut buf);
        match respone {
            Ok((_,_src_addr)) => {
                let _reply = String::from_utf8(buf.to_vec()).unwrap();
                println!("recieved from client ")
            }
            Err(_) =>()
        }
    } 
        
}

fn agent(socket : &UdpSocket){  // recieve from the client and send to the server based on turn
    let server_list = ["10.40.33.121:7879","10.40.46.106:7878"];
    let mut  i = 0;

    loop 
    {
        let mut buf = [0;1000];
        let (_, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        println!("Recieved successsfully from {}",src_addr);
        let client_request = String::from_utf8(buf.to_vec()).unwrap();
        println!("agent recieved client request : {}",client_request);
        // now we need to select which server to send to 
        let  x = socket.send_to(&mut buf, server_list[i]);
        match x {
            Ok(_) => println!("sent to server {}",i),
            Err(_) =>println!("server not responding")
        }
        if i >1 {
            i=0;
        }
        else {
            i = i +1;
        }
        


        let reply = String::from("Ack");
        let reply = reply.as_bytes();
        socket.send_to(reply, src_addr).expect("couldn't send data");
        println!("leaving agent");
    } 
        
        //send ack to client after executing request  
    
}
