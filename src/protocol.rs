use tungstenite::{connect, Message, Websocket};
use tungstenite::client::Client;
use native_tls::{TlsConnector, TlsStream};


struct UgcGatewayProtocol {
    open: bool,
    socket: WebSocket<TlsStream<TcpStream>>
}

impl UgcGatewayProtocol {

    fn connect(&self) -> PyResult<()> {
        let (mut socket, response) =
            connect(Url::parse("wss://ugc.renorari.net/api/v1/gateway").unwrap()).expect("なんらかの原因で接続できない");
        self.on_open();
        self.socket = Some(socket);
    }
      
    fn recv(&self) -> PyResult<String> {
        Ok(self.socket.read_message().unwrap())
    }
}
