
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
    


fn main(){
    //returns a new TCPListener Instance
    let listener=TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listener obj{:?}", listener);

    //represents an open conn between client server
    for stream in listener.incoming(){
            let stream=stream.unwrap();

            handle_connection(stream);
    }
}

//read data from the TCP stream and print it to inspect data being sent from the browser
fn handle_connection(mut stream:TcpStream){
    let buf_reader=BufReader::new(&mut stream);
    let https_request:Vec<_>=buf_reader
    .lines()
    .map(|result|result.unwrap())
    .take_while(|line|!line.is_empty())
    .collect();

    let status_line="HTTP/1.1 200 OK";
    let contents= fs::read("homepage.html").unwrap();
    let length=contents.len();



    let response= format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write(response.as_bytes()).unwrap();


}