use std::collections::VecDeque;
use std::task::Poll;
use std::{pin::Pin, task::ready};
use tokio::net::ToSocketAddrs;
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::UdpSocket,
};

pub struct AsyncUdpStream {
    // queue: VecDeque<u8>,
    inner: UdpSocket,
}

impl AsyncUdpStream {
    /// Default QueueSize: 4KByte
    const QUEUE_MAX: usize = 1024 << 4;
    pub async fn bind(addr: impl ToSocketAddrs) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self {
            // queue: VecDeque::with_capacity(Self::QUEUE_MAX),
            inner: socket,
        })
    }
    pub async fn connect(&self, addr: impl ToSocketAddrs) -> Result<(), std::io::Error> {
        self.inner.connect(addr).await?;
        Ok(())
    }
}

impl AsyncRead for AsyncUdpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.inner.poll_recv(cx, buf)
    }
}

impl AsyncWrite for AsyncUdpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        self.inner.poll_send(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        self.inner.poll_send_ready(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }
}
