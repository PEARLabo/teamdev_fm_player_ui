use serial2_tokio::SerialPort;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{AsyncRead, AsyncWrite};

mod async_net;
use async_net::AsyncUdpStream;
fn clear_buffer(port: &mut SerialPort) {
    port.discard_input_buffer().unwrap();
    port.discard_output_buffer().unwrap();
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortError {
    FailedToSerialPortOpen,
    FailedToUDPPortOpen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoKind {
    // (Tx Addr, Rx Addr)
    Udp((String, String)),
    Serial((String, u32)),
}

enum PortType {
    Udp(AsyncUdpStream),
    Serial(SerialPort),
}

pub struct AsyncIO {
    io_port: PortType,
}

impl AsyncIO {
    pub async fn open(kind: IoKind) -> Result<Self, PortError> {
        let io_port = match kind {
            IoKind::Udp(port) => {
                let maybe_socket = AsyncUdpStream::bind(port.1).await;

                if let Ok(socket) = maybe_socket {
                    if let Err(e) = socket.connect(port.0).await {
                        dbg!(e);
                        Err(PortError::FailedToUDPPortOpen)
                    } else {
                        Ok(PortType::Udp(socket))
                    }
                } else if let Err(e) = maybe_socket {
                    dbg!(e);
                    Err(PortError::FailedToUDPPortOpen)
                } else {
                    unreachable!()
                }
            }
            IoKind::Serial((port_name, baud_rate)) => {
                if let Ok(mut port) = SerialPort::open(port_name, baud_rate) {
                    clear_buffer(&mut port);
                    Ok(PortType::Serial(port))
                } else {
                    Err(PortError::FailedToSerialPortOpen)
                }
            }
        };
        io_port.map(|port| Self { io_port: port })
    }

    pub fn clear_buffer(&self) {
        match &self.io_port {
            PortType::Udp(_socket) => {
                // NO IMPLEMENT
            }
            PortType::Serial(port) => {
                port.discard_input_buffer().unwrap();
                port.discard_output_buffer().unwrap();
            }
        }
    }
}
impl AsyncWrite for AsyncIO {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        let this = unsafe { self.get_unchecked_mut() };
        match &mut this.io_port {
            PortType::Udp(socket) => std::pin::Pin::new(socket).poll_write(cx, buf),
            PortType::Serial(port) => std::pin::Pin::new(port).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        let this = unsafe { self.get_unchecked_mut() };

        match &mut this.io_port {
            PortType::Udp(socket) => std::pin::Pin::new(socket).poll_flush(cx),
            PortType::Serial(port) => std::pin::Pin::new(port).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        let this = unsafe { self.get_unchecked_mut() };

        match &mut this.io_port {
            PortType::Udp(socket) => std::pin::Pin::new(socket).poll_shutdown(cx),
            PortType::Serial(port) => std::pin::Pin::new(port).poll_shutdown(cx),
        }
    }
}
impl AsyncRead for AsyncIO {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        let this = unsafe { self.get_unchecked_mut() };

        match &mut this.io_port {
            PortType::Udp(socket) => std::pin::Pin::new(socket).poll_read(cx, buf),
            PortType::Serial(port) => std::pin::Pin::new(port).poll_read(cx, buf),
        }
    }
}
