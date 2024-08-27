use kioto_serial::SerialStream;
use tokio::io::AsyncReadExt;
pub async fn read_one_byte(port: &mut SerialStream) -> u8 {
    let mut buf = [0; 1];
    port.read_exact(&mut buf).await.unwrap();
    buf[0]
}
