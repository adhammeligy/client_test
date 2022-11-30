use std::{net::UdpSocket};
use std::time::{Duration, SystemTime};
use std::thread;
use std::sync::{Arc, Mutex};
use local_ip_address::local_ip;

fn main() {
    let my_local_ip = local_ip().unwrap();

    println!("This is my local IP address: {:?}", my_local_ip);
    
    let agent_socket = UdpSocket::bind(my_local_ip.to_string()+":7880").expect("couldnt bind to address");
    let server_socket = UdpSocket::bind(my_local_ip.to_string()+":7882").expect("couldnt bind to address");
    let response_socket = UdpSocket::bind(my_local_ip.to_string()+":7884").expect("couldnt bind to address");
    let awake_list = Arc::new(Mutex::new([true, true, true]));

    for i in 1500..2000
    {
        thread::spawn(move || {
            let port = format!("{}",i);
            let client_socket  = UdpSocket::bind(my_local_ip.to_string()+":"+&port).expect("couldn't bind to address");
            generate_request(&client_socket);
        });
    }
    // let thread_join_handle = thread::spawn(move || {
    //     generate_request(&client_socket);
    // });

    let awake_list_main = Arc::clone(&awake_list);



    let thread_join_handle2 = thread::spawn(move || {
        agent(&agent_socket, &awake_list_main);
    });

    let awake_list_main = Arc::clone(&awake_list);

    let thread_join_handle3 = thread::spawn(move || {
        agent_to_server(&server_socket, &awake_list_main);
    });
    let thread_join_handle4 = thread::spawn(move || {
        receive_responses(&response_socket);
    });

    // let _res = thread_join_handle.join();
    thread_join_handle2.join().unwrap();
    thread_join_handle3.join().unwrap();
    thread_join_handle4.join().unwrap();



}

fn generate_request(socket : &UdpSocket){
    let my_local_ip = local_ip().unwrap();
    let mut count = 0;
    let mut time::SystemTime;
    let mut avg_time = 0;
    loop {
        let start = SystemTime::now();
        let duration = Duration::from_secs(1);
        socket.set_read_timeout(Some(duration)).unwrap();
        let mut buf = [0;1000];
        let request = String::from("hi i am thesis");
        let timer = Duration::from_secs(1);
        
        socket.send_to(request.as_bytes(), my_local_ip.to_string()+":7880").expect("couldn't send data"); 
        let respone= socket.recv_from(&mut buf);
        match respone {
            Ok((_,_src_addr)) => {
                let _reply = String::from_utf8(buf.to_vec()).unwrap();
                println!("recieved from agent ")
            }
            Err(_) =>{println!("Request Dropped");}
        }
        let finish = start.elapsed();
        count = count + 1;
        time = time + finish;
        avg_time = time / count;
        println!("avg time = {}", avg_time);
        thread::sleep(timer);
    } 
        
}

fn agent(agent_socket : &UdpSocket, awake_list_fn : &Arc<Mutex<[bool;3]>>){  // recieve from the client and send to the server based on turn
    // let my_local_ip = local_ip().unwrap();
    let server_list = ["192.168.8.121:21543","192.168.8.122:21543","192.168.8.123:21543"];
    let mut  i = 0;
    let num_servers = 3;
    loop 
    {
        
        let mut buf = [0;1000];


        let (_, src_addr) = agent_socket.recv_from(&mut buf).expect("Didn't receive data");
        
        println!("agent recieved  successsfully from client : {}",src_addr);
        let client_request = String::from_utf8(buf.to_vec()).unwrap();
        println!("agent recieved client request : {}",client_request);
        
        let awake_list1 = {
            let awake_list1 = awake_list_fn.lock().unwrap();
            *awake_list1
        };
        
        // skip server that are asleep 
        if !awake_list1[i%num_servers] {
            i = i + 1;
        }

       
        agent_socket.send_to(&mut buf, &server_list[i%num_servers]).unwrap();
        
        println!("sent to server {}",i%num_servers);
       
        i = i +1;

        
        
        println!("leaving agent");
       
    }
     
        
        //send ack to client after executing request  
    
}


fn agent_to_server(server_socket : &UdpSocket, awake_list_fn : &Arc<Mutex<[bool;3]>>) {
    let mut buf = [0;1000];
    // let my_local_ip = local_ip().unwrap();
    let server_list = ["192.168.8.121:6000","192.168.8.122:6000","192.168.8.123:6000"];

    loop {
        let (_, srv_addr) = server_socket.recv_from(&mut buf).expect("Didn't receive data");

        // let server_status = String::from_utf8(buf.to_vec()).unwrap();
        // println!("{} is {} ##########&&&&&&&&&&&&&**********", srv_addr, server_status);

        for i in 0..server_list.len()
        {
            if srv_addr.to_string() == server_list[i]
            {
                let mut server_status = String::from_utf8(buf.to_vec()).unwrap();
                server_status = server_status.trim_matches(char::from(0)).to_string();
                let awake = String::from("awake");
                let sleep = String::from("sleep");

                println!("{} is {} ##########&&&&&&&&&&&&&**********", srv_addr, server_status);
                if server_status.eq(&sleep) 
                {
                    println!("{} is asleep #######################################", srv_addr);
                    
                    {
                        let mut awake_list1 = awake_list_fn.lock().unwrap();
                        awake_list1[i]=false;
                    }
                }
                else if server_status.eq(&awake){
                    println!("{} is awake #######################################", srv_addr);
                    {
                        let mut awake_list1 = awake_list_fn.lock().unwrap();
                        awake_list1[i]=true;
                    }
                }
                else {
                    println!("manga %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
                }
                break;
            }
        }
        
    }
    
}


fn receive_responses(response_socket : &UdpSocket)
{
    let mut total_requests = 0;
    let mut completed_requests = 0;
    let mut dropped_requests = 0;
    let server_list = ["192.178.8.121:21543","192.178.8.122:21543","192.178.8.123:21543"];
    let mut per_server_requests = [0,0,0];
    let mut buf = [0;1000];
    loop
    {
        let duration = Duration::from_secs(1);
        response_socket.set_read_timeout(Some(duration)).unwrap();

        let mangatos = response_socket.recv_from(&mut buf);
        match mangatos {
            Ok((_, src_addr)) =>  {
                let reply = String::from("Ack");
                let reply = reply.as_bytes();
                response_socket.send_to(reply, src_addr).expect("couldn't send data");
                total_requests = total_requests + 1;
                completed_requests = completed_requests+1;
                for i in 0..server_list.len()
                {
                    if src_addr.to_string() == server_list[i]
                    {
                        per_server_requests[i] = per_server_requests[i] + 1;
                        break;
                    }
                }
            },
            Err(_) => {
                dropped_requests = dropped_requests + 1;
                total_requests = total_requests + 1;
            }
        }
    }
}