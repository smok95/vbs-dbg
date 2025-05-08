use std::{io::{self, BufRead, Read}, net::{TcpListener, TcpStream}};

use dap::{Message, Request};
use serde_json::Value;

mod dap;

fn main() -> io::Result<()> {
    let mut builder = env_logger::Builder::from_default_env();
    builder.filter_level(log::LevelFilter::Debug).init();

    let port = 4711;
    let address = format!("127.0.0.1:{}", port);
    
    log::info!("DAP 서버 시작 - {}", address);

    // tcp서버 시작
    let listener = TcpListener::bind(address)?;

    if let Ok((stream, _)) = listener.accept() {
        handle_client(stream)?;
    }

    Ok(())
}

fn handle_client(mut stream:TcpStream) -> io::Result<()> {
    
    log::info!("클라이언트 연결됨");

    let length_keyword = "Content-Length: ";

    let mut reader = io::BufReader::new(&stream);

    // 메시지 처리
    loop {
        // Content-Length 헤더 읽기
        let mut header = String::new();
        let bytes_read = reader.read_line(&mut header)?;

        if bytes_read == 0 {
            // EOF, 연결 종료
            break;
        }   

        if !header.starts_with(length_keyword) {
            continue;
        }

        // Content-Length 값 파싱
        let content_length : usize = header[length_keyword.len()..].trim().parse().expect("유효한 Content-Length 값이 아님");

        // 빈줄 읽기
        let mut empty_line = String::new();
        reader.read_line(&mut empty_line)?;

        // 메시지 본문 읽기
        let mut buffer = vec![0; content_length];
        reader.read_exact(&mut buffer)?;

        // 받은 메시지 로깅
        let message = String::from_utf8_lossy(&buffer);

        log::debug!("받은 메시지: {}", message);
        match serde_json::from_str::<Message>(&message) {
            Ok(Message::Request(request)) => {
                handle_request(request)?;
            },
            Ok(Message::Response(response)) => {
                log::info!("Response : {:#?}", response);                
            },
            Ok(Message::Event(event)) => {
                log::info!("Event: {:#?}", event);
            }
            Err(e) => {
                log::error!("JSON 파싱 오류: {}", e);
            }
        }

    }

    Ok(())
}

fn handle_request(request: Request) -> io::Result<()> {
    log::info!("Request: {:#?}", request);
    Ok(())
}