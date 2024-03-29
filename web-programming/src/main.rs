


use std::time::Duration;
use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Write};
use std::io::BufRead;
use std::sync::{Arc, Mutex};

/*
    Response Syntax

    HTTP-Version Status-Code Reason-Phrase CRLF(carriage return and line feed)
    headers CRLF
    message-body

    ex: HTTP/1.1 200 OK\r\n\r\n
*/

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);

    // let http_request = buf_reader.lines()
    // .map(|result| result.unwrap())
    // .take_while(|lines| !lines.is_empty())
    // .collect::<Vec<String>>();
    // println!("Request: {:#?}", http_request);

    // SIMPLE RESPONSE

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();

    // SENDING HTML RESPONSE

    // let status_line = "HTTP/1.1 200 OK \r\n";
    // let contents = fs::read_to_string("index.html").unwrap();
    // let length = contents.len();
    // let response = format!("{} Contents-Length: {}\r\n\r\n{}",status_line,length,contents);
    // stream.write_all(response.as_bytes()).unwrap();
    // stream.flush().unwrap();

    // CHECKING PATH BEFORE SENDING RESPONSE

    let request_line = buf_reader.lines().next();

    let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            (Some("HTTP/1.1 200 OK \r\n"), Some("index.html"))
        }
        "GET /index HTTP/1.1" => (Some("HTTP/1.1 200 OK \r\n"), Some("index.html")),
        _ => (Some("HTTP/1.1 400 NOT FOUND \r\n"), Some("notFound.html")),
    };

    let contents = fs::read_to_string(file_name.unwrap()).unwrap();
    let response = format!("{} Contents-Length: {}\r\n\r\n{}",status_line.unwrap(),contents.len(),contents);
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main(){

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("HTTP Server running on localhost:8000/");

    // accepts single request and exits
    // let stream = listener.accept();

    // println!("The stream {:?} \nThe socket {:?}",stream.as_ref().unwrap().1,stream.as_ref().unwrap().0);

    // accepting multiple requests 10 to be exact
    // for i in 0..10{
    //     match listener.accept(){
    //         Ok((socket,addr))=> println!("Client Info {:?}",addr),
    //         Err(e)=> println!("Could not get client info")
    //     }
    // }

    let active_requests = Arc::new(Mutex::new(0));

    // always keep accepting incoming requests
    for stream in listener.incoming(){
        let active_requests = Arc::clone(&active_requests);
        let mut stream = stream.unwrap();


        thread::spawn(move||{
            let mut handled = false;
            {
                // acquiring lock inside block so that it is released once block ends
                let mut connection = active_requests.lock().unwrap();
                *connection += 1;
                if *connection >= 3{
                    let response = "HTTP/1.1 429 Too Many Requests \r\n";
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                    handled = true;
                }
            }

            if !handled {
                handle_connection(stream);
            }
            

            {
                let mut connection = active_requests.lock().unwrap();
                *connection -= 1;
            }

        });

    }
}