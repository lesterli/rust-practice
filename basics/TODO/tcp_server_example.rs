// 引入相关的Rust标准库std
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

// 处理tcp客户端的函数
fn handle_client(mut stream: TcpStream) {
    // 初始化100字节
    let mut data = [0 as u8; 100];
    // 读取客户端的数据
    while match stream.read(&mut data) {
        Ok(size) => {
            // 将客户端的数据echo返回
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

// 主函数
fn main() {
    // 监听本机的6666端口
    let listener = TcpListener::bind("127.0.0.1:6666").unwrap();
    // 打印提示信息
    println!("Server listening on port 6666");
    // 监听每个连接
    for stream in listener.incoming() {
        match stream {
            // 连接成功
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // 启动线程
                thread::spawn(move|| {
                    // 调用handle_client函数处理tcp连接
                    handle_client(stream)
                });
            }
            // 连接失败
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    // 关闭socket
    drop(listener);
}