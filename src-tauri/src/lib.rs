use std::net::SocketAddr;
use std::sync::Arc;
use tauri::State;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct UdpState(pub Arc<Mutex<Option<UdpSocket>>>);

#[tauri::command]
async fn initialize_udp(
    state: State<'_, UdpState>,
    local_addr: String,
    remote_addr: String,
) -> Result<(), String> {
    let local: SocketAddr = local_addr.parse().map_err(|e| format!("local addr parse error: {e}"))?;
    let remote: SocketAddr = remote_addr.parse().map_err(|e| format!("remote addr parse error: {e}"))?;

    let socket = UdpSocket::bind(local).await.map_err(|e| e.to_string())?;
    socket.connect(remote).await.map_err(|e| e.to_string())?;

    let mut guard = state.0.lock().await;
    *guard = Some(socket);

    Ok(())
}

#[tauri::command]
async fn send_udp(
    state: State<'_, UdpState>,
    message: String,
) -> Result<(), String> {
    let guard = state.0.lock().await;

    match guard.as_ref() {
        Some(socket) => {
            socket
                .send(message.as_bytes())
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        None => Err("UDP socket is not initialized".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(UdpState(Default::default()))
    .invoke_handler(tauri::generate_handler![
      initialize_udp,
      send_udp
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
