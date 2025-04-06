use std::{net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket}, thread};
use stunclient::StunClient;
use text_io::scan;

const STUN_ADDRESS: &str = "stun.12voip.com:3478";

fn get_stun_message(socket: &mut UdpSocket) -> String {
    // Resolve o hostname para obter um SocketAddr
    let stun_server = STUN_ADDRESS
        .to_socket_addrs()
        .expect("Falha ao resolver endereço STUN")
        .next()
        .expect("Nenhum endereço encontrado");
    
    // Cria o cliente STUN utilizando o endereço resolvido
    let client = StunClient::new(stun_server);
    
    // Realiza a consulta STUN e retorna o endereço público detectado
    match client.query_external_address(socket) {
        Ok(public_addr) => format!("Endereço público: {}", public_addr),
        Err(e) => format!("Erro ao consultar o servidor STUN: {}", e),
    }
}

fn main() {
    let mut socket = UdpSocket::bind("0.0.0.0:5000");
    let Ok(mut socket) = socket else {
        println!("Erro ao criar o socket UDP: {}", socket.unwrap_err());
        return;
    };

    let stun_addr = get_stun_message(&mut socket);
    println!("Seu endereço aberto é : {}", stun_addr);
    println!("Agora insira o endereço que deseja se conectar");
    let address : String =text_io::read!();
    println!("Conectando ao endereço: {}", address);

    socket.connect(address.clone()).expect("Erro ao conectar ao endereço fornecido");
    println!("Conectado ao endereço fornecido.");
    let  socket_clone = socket.try_clone().expect("Erro ao clonar o socket");
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match socket_clone.recv(&mut buffer) {
                Ok(size) => {
                    let message = String::from_utf8_lossy(&buffer[..size]);
                    println!("[{}]: {}",address, message);
                }
                Err(e) => {
                    println!("Erro ao receber mensagem: {}", e);
                    break;
                }
            }
        }
    });
    loop {
        let mut message = String::new();
        println!("Digite a mensagem para enviar (ou 'sair' para encerrar): ");
        std::io::stdin().read_line(&mut message).expect("Erro ao ler a mensagem");
        let message = message.trim();
        if message == "sair" {
            break;
        }
        socket.send(message.as_bytes()).expect("Erro ao enviar mensagem");
    }
    
}
