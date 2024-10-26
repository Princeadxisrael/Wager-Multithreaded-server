use std::net::TcpListener;


fn main(){
    let addr:String=String::from("127.0.0.1:7878");
    let listener=TcpListener::bind(addr).unwrap();
    println!("Listener obj{:?}", listener);

    for stream in listener.incoming(){
            let stream=stream.unwrap();

            println!("connection established")
    }
}