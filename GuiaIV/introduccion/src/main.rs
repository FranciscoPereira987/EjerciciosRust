//Como es todo dentro del mismo programa
//voy a pasarme variables de forma tranqui
use std::io::prelude::*;

use std::net::{TcpListener, TcpStream};

use std::sync::mpsc;

use std::io::BufReader;

use std::thread;

fn main() {
    let addres = "0.0.0.0:4560";
    let (sender, handler) = create_thread(addres);
    println!("Termine de preparar el thread");
    set_up_padre(addres, sender);
    if handler.join().is_err() {
        println!("Error cerrando el thread");
    }
}

fn set_up_padre(addres: &str, sender: mpsc::Sender<usize>) {
    let stream_result = TcpListener::bind(addres);
    if sender.send(0).is_err() {
        println!("Error enviando por el sender");
    }
    println!("Le pase clave al thread");
    if let Ok(stream) = stream_result {
        loop {
            let connection_result = stream.accept();
            if let Ok(connection) = connection_result {
                let stream = connection.0;
                let mut reader = BufReader::new(stream);
                let mut string = String::new();
                if let Ok(_) = reader.read_line(&mut string) {
                    println!("Recibido: {}", string);
                }
                let stream_result = TcpStream::connect(connection.1);
                if let Ok(mut stream) = stream_result {
                    if stream.write(&("Hola hijo".as_bytes())).is_err() {
                        println!("Error respondiendole a mi hijo");
                    }
                }
            }
        }
    } else {
        println!("Algo salio mal al abrir el lector");
    }
}

fn create_thread(addres: &str) -> (mpsc::Sender<usize>, thread::JoinHandle<()>) {
    let (sender, reciever) = mpsc::channel();
    let addres_string = String::from(addres);
    let handler = thread::spawn(move || {
        if reciever.recv().is_err() {
            println!("Problema leyendo del channel");
        }
        let stream_result = TcpStream::connect(addres_string);
        if let Ok(mut stream) = stream_result {
            if stream.write(&("Buen dia papa!\n".as_bytes())).is_err() {
                println!("No se pudo enviar el saludo al padre")
            }
            let mut reader = BufReader::new(stream);
            let mut string = String::new();
            if let Ok(_) = reader.read_line(&mut string) {
                println!("Recibido hijo: {}", string);
            }
        }
    });

    (sender, handler)
}
