use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};
use tokio_util::codec::LengthDelimitedCodec;
use crate::service::io::Channel;
use std::net::SocketAddr;
use bytes::Bytes;
use std::io::Error;

pub mod io;

#[async_trait]
pub trait ServiceChannel {
    async fn send(&mut self, payloads: Option<Vec<Bytes>>) -> Result<(), Error>;
}

#[async_trait]
pub trait ServiceHandler {
    async fn handle(&self, mut socket: TcpStream);
}

#[async_trait]
pub trait Service {
    async fn serve(&self) -> std::result::Result<(), Box<dyn std::error::Error>>;
}

pub trait ServiceCodec {
    fn write_frame<T: AsyncWrite>(&self, io: T) -> FramedWrite<T, LengthDelimitedCodec>;
    fn read_frame<T: AsyncRead>(&self, io: T) -> FramedRead<T, LengthDelimitedCodec>;
}