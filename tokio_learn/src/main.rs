extern crate tokio;
extern crate futures;

// use tokio::io;
// use tokio::net::TcpStream;
// use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    // let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    // stream.write_all(b"hello world\n").await.unwrap();
    // println!("wrote to stream");

    // let handle = tokio::spawn(async {
    //     println!("doing some work, asynchronously");
    //     "result of the computation"
    // });
    // let res = handle.await;
    // println!("got {:?}", res);

    // Again, no work actually happens when creating the server variable.
    // You have to actually await it or otherwise spawn it on the executor using e.g. tokio::spawn.
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    // let server = async move {
    //     let mut incoming = listener.incoming();
    //     while let Some(socket_res) = incoming.next().await {
    //         match socket_res {
    //             Ok(socket) => {
    //                 println!("Accepted connection from {:?}", socket.peer_addr());
    //             }
    //             Err(err) => {
    //                 println!("Accept error: {:?}", err);
    //             }
    //         }
    //     }
    // };
    let server = {
        async move {
            let mut incoming = listener.incoming();
            while let Some(conn) = incoming.next().await {
                match conn {
                    Err(e) => eprintln!("Accept error: {:?}", e),
                    Ok(mut sock) => {
                        tokio::spawn(async move {
                            let (mut reader, mut writer) = sock.split();

                            match tokio::io::copy(&mut reader, &mut writer).await {
                                Ok(amt) => {
                                    println!("Wrote {} bytes", amt);
                                }
                                Err(err) => {
                                    eprintln!("IO error: {:?}", err);
                                }
                            }
                        });
                    }
                }
            }
        }
    };

    println!("Server running on localhost:6142");

    server.await;
}
