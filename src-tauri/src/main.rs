// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod service;
use serde::{Serialize, Deserialize};

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
    app::db_save(key.to_owned(),cluster.to_owned());
    return "Ok".to_owned();
}

#[tauri::command]
fn querycluster(req: &str) -> String{
   let key = "cluster/records";

   let result = app::db_query(key.to_owned());
   if result.is_ok(){
     return result.unwrap();
   }

   return "Err".to_owned();
}


#[tauri::command]
fn saveselectedinstance(req: &str) -> String{
    let key = "cluster/instance";
    app::db_save(key.to_owned(),req.to_owned());
    return "Ok".to_owned();
}

#[tauri::command]
fn queryselectedinstance(req: String) -> String{
   let key = "cluster/instance";

   let result = app::db_query(key.to_owned());
   if result.is_ok(){
     return result.unwrap();
   }

   return "Err".to_owned();
}


#[tauri::command]
async fn etcd_all_key() -> Option<Vec<service::etcd::JSonKeyValue>>{
  let result =  app::etcd_get_all("".to_string()).await;
  match result {
    Ok(records) => {
      return Some(records);
    },
    Err(_) => {},
  }

  return None;
}

#[tauri::command]
async fn etcd_delete_key(req: String) -> Option<String>{
  let result =  app::etcd_delete(req.to_string()).await;
  match result {
      Ok(ret) => {
         return Some(ret);
      },
      Err(_) => {

      },
  };

  return None;
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct DtoKeyValue {
  pub root: String,
  pub name: String,
  pub ttl: i64,
  pub isdir: bool,
  pub value: String,
}

#[tauri::command]
async fn etcd_put_key(req: DtoKeyValue) -> Option<String>{
  let mut key = req.root;
  if !key.as_str().ends_with("/") {
    key = key + "/";
  }

  if req.isdir {
    key = key + req.name.as_str();
    key = key + "/";
  } else {
    key = key + req.name.as_str();
  }

  app::etcd_put(key,req.value).await;

  return Some("Ok".to_owned());
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct DtoSimpleKeyValue {
    pub key: String,
    pub value: String,
}

#[tauri::command]
async fn etcd_simpleput_key(req: DtoSimpleKeyValue) -> Option<String>{
    app::etcd_put(req.key,req.value).await;
    return Some("Ok".to_owned());
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![echo,savecluster,querycluster,
            queryselectedinstance,saveselectedinstance,etcd_all_key,etcd_delete_key,etcd_put_key,etcd_simpleput_key
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}