#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::io::Write;
use etcd_client::Permission;
use serde::{Serialize, Deserialize};
use crate::app;
use crate::types::dto::*;
#[tauri::command]
pub async fn list_roles() -> Option<Vec<String>>{
    let roles = app::etcd_role_list().await;
    return roles;
}


#[tauri::command]
pub fn echo(req: &str) -> String {
    let recived = format!("{}", req);
    println!("!! {}",&recived);
    recived
}

#[tauri::command]
pub fn savecluster(cluster: &str) -> String{
    let key = "cluster/records";
    print!("{}",cluster);
    app::db_save(key.to_owned(),cluster.to_owned());
    return "Ok".to_owned();
}

#[tauri::command]
pub fn querycluster(req: &str) -> String{
    let key = "cluster/records";

    let result = app::db_query(key.to_owned());
    if result.is_ok(){
        return result.unwrap();
    }

    return "Err".to_owned();
}


#[tauri::command]
pub fn saveselectedinstance(req: &str) -> String{
    let key = "cluster/instance";
    app::db_save(key.to_owned(),req.to_owned());
    return "Ok".to_owned();
}

#[tauri::command]
pub fn queryselectedinstance(req: String) -> String{
    let key = "cluster/instance";

    let result = app::db_query(key.to_owned());
    if result.is_ok(){
        return result.unwrap();
    }

    return "Err".to_owned();
}


#[tauri::command]
pub async fn etcd_all_key() -> Option<Vec<JSonKeyValue>>{
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
pub async fn etcd_delete_key(req: String) -> Option<String>{
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

#[tauri::command]
pub async fn etcd_put_key(req: DtoKeyValue) -> Option<String>{
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

#[tauri::command]
pub async fn etcd_simpleput_key(req: DtoSimpleKeyValue) -> Option<String>{
    app::etcd_put(req.key,req.value).await;
    return Some("Ok".to_owned());
}

#[tauri::command]
pub async fn list_roles_permission(req: String) -> Option<Vec<PermissionValue>>{
    let roles = app::etcd_role_permissions(req).await;
    match roles {
        Some(role_list) => {
            let mut perms = Vec::<PermissionValue>::new();
            role_list.iter().for_each(|it|{
               let perm = PermissionValue{
                   ty: it.get_type(),
                   key: it.key_str().unwrap().to_string(),
                   range_end: it.range_end_str().unwrap().to_string(),
                   with_prefix: it.is_prefix(),
                   with_from_key: it.is_from_key(),
               };
               perms.push(perm);
            });
            return Some(perms);
        },
        None => {
        }
    }
    return None;
}

#[tauri::command]
pub async fn etcd_add_role(req: String) -> Option<String>{
    let response = app::etcd_add_role(req).await;
    match response {
        Ok(result) => {
            return Some("".to_owned());
        }
        _ => {
            return None;
        }
    }
}

#[tauri::command]
pub async fn etcd_del_role(req: String) -> Option<String>{
    let response = app::etcd_del_role(req).await;
    match response {
        Ok(result) => {
            return Some("".to_owned());
        }
        _ => {
            return None;
        }
    }
}

#[tauri::command]
pub async fn etcd_role_grant_perimssions(role: String,ty :String, pathx :String) -> Option<String>{
    let response = app::etcd_grant_role_permissions(role,ty,pathx).await;
    match response {
        Ok(result) => {
            return Some("".to_owned());
        }
        _ => {
            return None;
        }
    }
}

#[tauri::command]
pub async fn etcd_role_revoke_perimssions(role: String,ty :String, pathx :String, range_end: String) -> Option<String>{
    let response = app::etcd_revoke_role_permissions(role,ty,pathx,range_end).await;
    match response {
        Ok(result) => {
            return Some("".to_owned());
        }
        _ => {
            return None;
        }
    }
}

#[tauri::command]
pub async fn etcd_user_list() -> Option<Vec<String>>{
    let response = app::etcd_user_list().await;
    return response;
}

#[tauri::command]
pub async fn etcd_user_role_list(name :String) -> Option<Vec<String>>{
    let response = app::etcd_user_role_list(name).await;
    return response;
}

#[tauri::command]
pub async fn etcd_user_add(name :String,password: String) -> Option<()>{
    let response = app::etcd_user_add(name,password).await;
    match response {
        Ok(result) => {
            return Some(());
        }
        _ => {}
    }
    None
}

#[tauri::command]
pub async fn etcd_user_delete(name :String) -> Option<()>{
    let response = app::etcd_user_delete(name).await;
    match response {
        Ok(result) => {
            return Some(());
        }
        _ => {}
    }
    None
}

#[tauri::command]
pub async fn user_grant_role(user :String,role :String) -> Option<()>{
    let response = app::user_grant_role(user,role).await;
    match response {
        Ok(result) => {
            return Some(());
        }
        _ => {}
    }
    None
}

#[tauri::command]
pub async fn user_revoke_role(user :String,role :String) -> Option<()>{
    let response = app::user_revoke_role(user,role).await;
    match response {
        Ok(result) => {
            return Some(());
        }
        _ => {}
    }
    None
}