// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn wow(name: &str) -> String {
    format!("Wow, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,wow])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod test {
    type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
    type GenericResult<T> = Result<T,GenericError>;
    use etcd_client::{Client, Error};

    #[tokio::test]
    async fn test_etcd() -> GenericResult<()>{
        let address = "192.168.11.214:30380";
        let mut client = Client::connect([address], None).await?;
        let val = client.get("/udscctlplane/config/192.168.11.233:allinone/prop",None).await?;
        Ok(())
    }
}