use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
};

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();
    println!("lister: {:?}", listener);

    while let Ok((stream, _)) = listener.accept() {
        let stream0 = stream.try_clone().unwrap();
        let mut reader = BufReader::new(stream0);
        let mut writer = BufWriter::new(stream);

        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        trim_newline(&mut buf);
        println!("from client: {}", buf);

        let s = format!("[{}]", buf);
        println!("to client: {}", s);
        write!(&mut writer, "{}\n", s).unwrap();
        writer.flush().unwrap();
    }
}
