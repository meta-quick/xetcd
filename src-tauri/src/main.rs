// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod service;

#[tauri::command]
fn echo(req: &str) -> String {
   let recived = format!("{}", req);
   println!("!! {}",&recived);
   recived
}

#[tauri::command]
fn savecluster(cluster: &str) -> String{
    let key = "cluster/records";
    print!("{}",cluster);
    app::save(key.to_owned(),cluster.to_owned());
    return "Ok".to_owned();
}

#[tauri::command]
fn querycluster(req: &str) -> String{
   let key = "cluster/records";

   let result = app::query(key.to_owned());
   if result.is_ok(){
     return result.unwrap();
   }

   return "Err".to_owned();
}


#[tauri::command]
fn saveselectedinstance(req: &str) -> String{
    let key = "cluster/instance";
    app::save(key.to_owned(),req.to_owned());
    return "Ok".to_owned();
}

#[tauri::command]
fn queryselectedinstance(req: &str) -> String{
   let key = "cluster/instance";

   let result = app::query(key.to_owned());
   if result.is_ok(){
     return result.unwrap();
   }

   return "Err".to_owned();
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![echo,savecluster,querycluster,
            queryselectedinstance,saveselectedinstance
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}