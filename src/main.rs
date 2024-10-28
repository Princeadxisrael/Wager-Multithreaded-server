
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
    let request_line=buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename)= if request_line == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK", "homepage.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let contents= fs::read_to_string(filename).unwrap();
    let length=contents.len();
    let response= format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write(response.as_bytes()).unwrap();
    

}