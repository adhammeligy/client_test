use std::{net::UdpSocket};
use std::time;
use std::thread;
fn main() {
    let socket  = UdpSocket::bind("127.0.0.1:7881").expect("couldn't bind to address");
    let sock = UdpSocket::bind("127.0.0.1:7880").expect("couldnt bind to address");
    let sc = UdpSocket::bind("127.0.0.1:7882").expect("couldnt bind to address");


    
    let thread_join_handle = thread::spawn(move || {
        generate_request(&socket);
    });

    let thread_join_handle2 = thread::spawn(move || {
        agent(&sock, &sc);
    });

    let _res = thread_join_handle.join();
    thread_join_handle2.join().unwrap();


}

fn generate_request(socket : &UdpSocket){
    loop {
        let duration = time::Duration::from_secs(1);
        socket.set_read_timeout(Some(duration)).unwrap();
        let mut buf = [0;1000];
        let request = String::from("hi i am meligy");
        let timer = time::Duration::from_secs(5);
        
        socket.send_to(request.as_bytes(), "127.0.0.1:7880").expect("couldn't send data"); 
        let respone= socket.recv_from(&mut buf);
        match respone {
            Ok((_,_src_addr)) => {
                let _reply = String::from_utf8(buf.to_vec()).unwrap();
                println!("recieved from agent ")
            }
            Err(_) =>()
        }
        thread::sleep(timer);
    } 
        
}

fn agent(socket : &UdpSocket, sock : &UdpSocket){  // recieve from the client and send to the server based on turn
    let server_list = ["127.0.0.1:7879"];
    let mut awake_list = [true, true, true];
    let mut  i = 0;
    let num_servers = 1;
    loop 
    {
        
        let mut buf = [0;1000];
        
        let (_, srv_addr) = sock.recv_from(&mut buf).expect("Didn't receive data");
        
        while i < num_servers {
            if srv_addr.to_string() == server_list[i]
            {
                if buf == "0".as_bytes() {
                    println!("Server is now sleeping : {}",srv_addr);
                    awake_list[i] = false;
                }
                else {
                    println!("Server is now awake : {}",srv_addr);
                    awake_list[i] = true;
                }
            }
        }


        let (_, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        
        println!("agent recieved  successsfully from client : {}",src_addr);
        let client_request = String::from_utf8(buf.to_vec()).unwrap();
        println!("agent recieved client request : {}",client_request);
        // now we need to select which server to send to 
        if !awake_list[i] {
            i = i + 1;
        }
        let  x = socket.send_to(&mut buf, server_list[i%num_servers]);
        match x {
            Ok(_) => println!("sent to server {}",i%num_servers),
            Err(_) =>()
        }
       
        i = i +1;

        let duration = time::Duration::from_secs(6);
        socket.set_read_timeout(Some(duration)).unwrap();

        let mangatos = socket.recv_from(&mut buf);
        match mangatos {
            Ok((_, _src_addr)) =>  {let reply = String::from("Ack");
            let reply = reply.as_bytes();
            socket.send_to(reply, src_addr).expect("couldn't send data");},
            Err(_) => {let reply = String::from("Request dropped");
            let reply = reply.as_bytes();
            socket.send_to(reply, src_addr).expect("couldn't send data");}
        }
        
        println!("leaving agent");
        let sec = time::Duration::from_secs(1);
        thread::sleep(sec);
    }
     
        
        //send ack to client after executing request  
    
}
