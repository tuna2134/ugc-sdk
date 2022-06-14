use tungstenite::{connect, Message, Websocket};
use tungstenite::client::Client;
use native_tls::{TlsConnector, TlsStream};


struct UgcGatewayProtocol {
    open: bool,
    socket: WebSocket<TlsStream<TcpStream>>
}

impl UgcGatewayProtocol {

    fn connect(&mut self) -> Result<()> {
        let (mut socket, response) =
            connect(Url::parse("wss://ugc.renorari.net/api/v1/gateway").unwrap()).expect("なんらかの原因で接続できない");
        self.socket = Some(socket);
    }
      
    fn recv(&self) -> Result<String> {
        Ok(self.socket.read_message().unwrap())
    }
}
