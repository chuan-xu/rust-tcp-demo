// 加载读写库
use std::io::{Write, BufReader, BufRead};
use std::net::TcpStream;

fn main() {
    // 连接tcp服务
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("[-] tcp连接失败");

    // 发送消息，错误处理
    match writeln!(&mut stream, "hello world") {
        Ok(_) => {
            // 实用bufreader读取数据
            let mut reader = BufReader::new(&stream);
            // 将数据存入buf中
            let mut buf = String::new();
            // 读取一行数据，遇到"\n"结束
            match reader.read_line(&mut buf) {
                //打印信息
                Ok(_) => println!("[+] {:?}", buf.replace("\n", "")),
                //错误分支，打印错误
                Err(e) => println!("[-] 数据读取失败: {:?}", e),
            }
        },
        // 错误分支，打印错误
        Err(e) => println!("[-] 数据写入失败: {:?}", e),
    }
    
}
