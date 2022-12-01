use std::{net::UdpSocket};
use std::time::{Duration, SystemTime};
use std::thread;
use std::sync::{Arc, Mutex};
use local_ip_address::local_ip;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    
    //file.write_all(b"HelloHelloHello\nxyz\tabc").unwrap();
    let my_local_ip = local_ip().unwrap();

    //could try to find a way to create a folder first
    //also could add a way to (by loop over error in match to create a new statistics file everytime we rerun);

    
    println!("This is my local IP address: {:?}", my_local_ip);
    
    let agent_socket = UdpSocket::bind(my_local_ip.to_string()+":7880").expect("couldnt bind to address");
    let server_socket = UdpSocket::bind(my_local_ip.to_string()+":7882").expect("couldnt bind to address");
    let response_agent_socket = UdpSocket::bind(my_local_ip.to_string()+":7884").expect("couldnt bind to address");
    let response_srv_socket = UdpSocket::bind(my_local_ip.to_string()+":7885").expect("couldnt bind to address");
    let awake_list = Arc::new(Mutex::new([true, true, true]));

    
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

    let mut file1 = File::create("statistics.txt").expect("Error encountered while creating file!");

    let thread_join_handle4 = thread::spawn(move || {
        receive_responses(&response_agent_socket,&response_srv_socket, &file1);
    });

    for i in 1500..2000
    {
        // let timer = Duration::from_secs(1);
        thread::spawn(move || {
            //let path = String::from("_");
            let port = format!("{}",i);
            //println!()
            let mut file = File::create("genreq/".to_string()+ &port + ".txt").expect("Error encountered while creating file!");
            let client_socket  = UdpSocket::bind(my_local_ip.to_string()+":"+&port).expect("couldn't bind to address");
            generate_request(&client_socket, &file);
        });
        // thread::sleep(timer);
    }
    // let _res = thread_join_handle.join();
    thread_join_handle2.join().unwrap();
    thread_join_handle3.join().unwrap();
    thread_join_handle4.join().unwrap();



}

fn generate_request(socket : &UdpSocket, mut file : &File){
    let my_local_ip = local_ip().unwrap();
    let mut count: f64 = 0.0;
    let mut time: f64 = 0.0;
    let mut avg_time: f64 = 0.0;
    loop {
        let start = SystemTime::now();
        let duration = Duration::from_millis(10);
        socket.set_read_timeout(Some(duration)).unwrap();
        let mut buf = [0;1000];
        let request = String::from("hi i am thesis");
        let timer = Duration::from_secs(5);
        
        socket.send_to(request.as_bytes(), my_local_ip.to_string()+":7880").expect("couldn't send data"); 
        let respone= socket.recv_from(&mut buf);
        match respone {
            Ok((_,_src_addr)) => {
                let _reply = String::from_utf8(buf.to_vec()).unwrap();
                // println!("recieved from agent ")
            }
            Err(_) =>{println!("Request Dropped");}
        }
        let mut finish: f64 = 0.0;
        
        match start.elapsed() {
            Ok(elapsed) => {
                finish = elapsed.as_millis() as f64;
            }
            Err(e) => {
                println!("Error 3'areeb : {}", e);
            }
        }

        count = count + 1.0;
        time = time + finish;
        avg_time = time / count;
        // println!("avg time = {}", avg_time);
        
        if count % 2.0 == 0.0 {
            //write to file
            let avg_str = format!("Average request time ={}", avg_time);
            file.write_all(avg_str.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        thread::sleep(timer);
    } 
        
}

fn agent(agent_socket : &UdpSocket, awake_list_fn : &Arc<Mutex<[bool;3]>>){  // recieve from the client and send to the server based on turn
    let my_local_ip = local_ip().unwrap();
    let server_list = ["10.40.40.45:21543","192.168.8.122:21543","192.168.8.123:21543"];
    let mut  i = 0;
    let num_servers = 3;
    loop 
    {
        
        let mut buf = [0;1000];


        let (_, src_addr) = agent_socket.recv_from(&mut buf).expect("Didn't receive data");
        
        // println!("agent recieved  successsfully from client : {}",src_addr);
        let client_request = String::from_utf8(buf.to_vec()).unwrap();
        // println!("agent recieved client request : {}",client_request);
        
        let awake_list1 = {
            let awake_list1 = awake_list_fn.lock().unwrap();
            *awake_list1
        };
        
        // skip server that are asleep 
        if !awake_list1[i%num_servers] {
            i = i + 1;
        }

    //    println!("server = {}%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%", server_list[i%num_servers]);
        agent_socket.send_to(&mut buf, &server_list[i%num_servers]).unwrap();
        
        agent_socket.send_to(src_addr.to_string().as_bytes(), my_local_ip.to_string()+":7884").expect("couldn't send data");
        // println!("SENT {} TO {}",src_addr, my_local_ip.to_string()+":7884");

        // println!("sent to server {}",i%num_servers);
       
        i = i +1;

        
        
        // println!("leaving agent");
       
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

                // println!("{} is {} ##########&&&&&&&&&&&&&**********", srv_addr, server_status);
                if server_status.eq(&sleep) 
                {
                    // println!("{} is asleep #######################################", srv_addr);
                    
                    {
                        let mut awake_list1 = awake_list_fn.lock().unwrap();
                        awake_list1[i]=false;
                    }
                }
                else if server_status.eq(&awake){
                    // println!("{} is awake #######################################", srv_addr);
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


fn receive_responses(agent_socket : &UdpSocket,response_socket : &UdpSocket, mut file : &File)
{
    let my_local_ip = local_ip().unwrap();

    // let mut count: f64 = 0.0;
    let mut time: f64 = 0.0;
    let mut avg_time: f64 = 0.0;

    let mut total_requests = 0;
    let mut completed_requests = 0;
    let mut dropped_requests = 0;
    let server_list = ["10.40.40.45:21543","192.178.8.122:21543","192.178.8.123:21543"];
    let mut per_server_requests = [0,0,0];
    //response_socket.connect(my_local_ip.to_string()+":21543").unwrap();
    agent_socket.connect(my_local_ip.to_string()+":7880").unwrap();
    loop
    {
        let mut buf = [0;1000];
        let mut buffer:[u8; 1000] = [0;1000];
        
        agent_socket.recv(&mut buffer).unwrap();
        let start = SystemTime::now();
        let mut reply = String::from_utf8(buffer.to_vec()).unwrap();
        reply = reply.trim_matches(char::from(0)).to_string();
        //reply is the client ip address
        let duration = Duration::from_millis(10);
        response_socket.set_read_timeout(Some(duration)).unwrap();
        //receive ACK from server
        let mangatos = response_socket.recv_from(&mut buf);
        match mangatos {
            Ok((_, src_addr)) =>  {
                // println!("RECEIVED ACK FROM {}$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$", src_addr);
                total_requests = total_requests + 1;
                completed_requests = completed_requests+1;
                for i in 0..server_list.len()
                {
                    // println!("{}#########################################", src_addr);
                    if src_addr.to_string() == server_list[i]
                    {
                        per_server_requests[i] = per_server_requests[i] + 1;
                        break;
                    }
                }
                
                
                response_socket.send_to("Ack".as_bytes(), reply).expect("couldn't send data");
                
                let mut finish: f64 = 0.0;
        
                match start.elapsed() {
                    Ok(elapsed) => {
                        finish = elapsed.as_millis() as f64;
                    }
                    Err(e) => {
                        println!("Error 3'areeb : {}", e);
                    }
                }

                //count = count + 1.0;
                time = time + finish;
                //avg_time = time / count;
                //println!("avg time = {}", avg_time);
                
                // if count % 2.0 == 0.0 {
                //     //write to file
                //     let avg_str = format!("Average request time ={}", avg_time);
                //     file.write_all(avg_str.as_bytes()).unwrap();
                //     file.write_all(b"\n").unwrap();
                // }
            
            },
            Err(_) => {
                dropped_requests = dropped_requests + 1;
                total_requests = total_requests + 1;
            }
        }
        // println!("################################################################");
        if total_requests % 100 == 0 {
            //write to file
            //add avg time per request here
            avg_time = time / total_requests as f64;
            let percentage = completed_requests as f64 / total_requests as f64;
            let mut server_loads = [0.0,0.0,0.0];
            let statistics = format!("Total ={}\nCompleted ={}\nDropped ={}\nSuccess Rate ={}%\nAverage Time per Request ={}\n", total_requests, completed_requests, dropped_requests, percentage, avg_time);
            file.write_all(statistics.as_bytes()).unwrap();
            for i in 0..server_loads.len()
            {
                server_loads[i] = per_server_requests[i] as f64 / total_requests as f64;
                let server_statistics = format!("Server {} requests = {}\nServer {} load = {}\n", i+1,per_server_requests[i],i+1,server_loads[i]);
                file.write_all(server_statistics.as_bytes()).unwrap();
            }
            
            file.write_all("\n".as_bytes()).unwrap();
        }
        
    }
}