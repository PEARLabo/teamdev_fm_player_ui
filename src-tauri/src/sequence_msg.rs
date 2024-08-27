use tokio::io::{AsyncReadExt, AsyncWriteExt};
pub async fn sequence_msg<R: tauri::Runtime>(
    first_byte: u8,
    port: &mut kioto_serial::SerialStream,
    app_handle: &impl tauri::Manager<R>,
) {
    let msg_flag = first_byte & 0xf;
    let len = (first_byte >> 4) as usize;
    let mut buf = vec![0; len];
    port.read_exact(&mut buf).await.unwrap();
    println!("{:?}", buf);
}
