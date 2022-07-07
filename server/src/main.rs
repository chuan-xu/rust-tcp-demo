// 加载读写库
use std::io::{Read, Write, BufReader, BufRead};
// 加载网络相关库
use std::net::{TcpListener, TcpStream};

fn main() {
    // 绑定监听服务
    let listener = TcpListener::bind("127.0.0.1:8080").expect("[-] tcp监听服务绑定失败");

    // 接受连接并串行处理
    for stream in listener.incoming() {
        // 错误匹配处理
        match stream {
            // handle_client错误判断打印
            Ok(s) => if let Err(e) = handle_client(s) {
                println!("{:?}", e);
            },
            // result err分支，打印错误
            Err(e) => println!("[-] 连接失败: {:?}", e),
        }
    }
}

// 信息处理方法
fn handle_client(mut stream: TcpStream) -> Result<(), String> {
    // 实用bufreader读取数据
    let mut reader = BufReader::new(&stream);
    // 将数据存入buf中
    let mut buf = String::new();
    // 读取一行数据，遇到"\n"结束
    reader
        .read_line(&mut buf)
        .map_err(|e| format!("[-] tcp读取失败: {:?}", e))?;
    // 打印数据
    println!("[+] {:?}", buf.replace("\n", ""));
    // 返回消息给client
    writeln!(&mut stream, "hello client from server").map_err(|e| format!("[-] echo失败: {:?}", e))?;
    // result 返回ok
    Ok(())
}
