use std::io::Write;
use etcd_client::Permission;
use crate::service::db::storage::Keydb;
use crate::service::etcd;
use crate::types::dto::*;
use serde_json::from_str;
use redb::Error;

pub fn db_save(key: String,val: String){
   let db = Keydb::open().unwrap();
   db.put(&key,&val).unwrap();
}
pub fn db_query(key: String) -> Result<String,Error>{
   let db = Keydb::open();
   if let Some(db) = db {
      return db.get(&key);
   }

   return Err(Error::Corrupted("".into()).into());
}

pub fn db_remove(key: String){
   let db = Keydb::open();
   if let Some(db) = db {
      db.remove(&key).unwrap();
   }
}

fn get_etcd_instance() -> Option<EtcdConfig>{
   let key = "cluster/instance";
   let result = db_query(key.to_owned());

   if let Ok(url) = result {
      let etcd_config :Result<Vec<EtcdConfig>,serde_json::Error> = from_str(&url);
      if let Ok(config) = etcd_config {
         let item = config.first();
         return Some(item.unwrap().clone());
      }
   }

   return None;
} 

pub async fn etcd_put(key: String,value: String){
   let mut etcd = etcd::client::Etcd::new();
   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;

      if let Ok(true) = state {
         let kv = SimpleKeyValue { key, value};
         let response =  etcd.put(kv).await;
         match response {
             Ok(_) => {},
             Err(_) => {},
         }
      }
   } 
}

pub async fn etcd_get(key: String) -> Result<JSonKeyValue,Error>{
   let mut etcd = etcd::client::Etcd::new();
   let urls = get_etcd_instance();
   if let Some(config) = urls {
      let state = etcd.open(config).await;

      if let Ok(true) = state {
         let response =  etcd.get(key).await;
         match response {
             Ok(result) => {return Ok(result)},
             Err(_) => { return Err(Error::Corrupted("".into()).into())},
         }
      } else {
         return Err(Error::Corrupted("".into()).into());
      }
   }

   return Err(Error::Corrupted("".into()).into());
}

pub async fn etcd_get_all(key: String) -> Result<Vec<JSonKeyValue>,Error>{
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();
   if let Some(config) = urls {
      let state = etcd.open(config).await;

      if let Ok(true) = state {
         let response =  etcd.get_all(key).await;
         match response {
             Ok(result) => {
               return Ok(result)
            },
             Err(_) => { return Err(Error::Corrupted("".into()).into())},
         }
      } else {
         return Err(Error::Corrupted("".into()).into());
      }
   }

   return Err(Error::Corrupted("".into()).into());
}

pub async fn etcd_delete(key: String)  -> Result<String,etcd_client::Error> {
   let mut etcd = etcd::client::Etcd::new();
   
   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let kv = SimpleKeyValue { key: key, value: "".to_string() };
         let response =  etcd.remove(kv).await;
         match response {
             Ok(()) => {
               return Ok("remove".to_owned());
            },
             Err(_) => { return Err(etcd_client::Error::InvalidArgs("".into()))},
         }
     }
   }
   Ok("removed".into())
}

pub async fn etcd_role_list()  -> Option<Vec<String>> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response =  etcd.list_roles().await;

         match response {
            Some(result) => {
               return Some(result);
            }
            _ => {

            }
         }
      }
   }
   None
}

pub async fn etcd_role_permissions(role: String)  -> Option<Vec<Permission>> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response =  etcd.get_role_permission(role).await;

         return response;
      }
   }
   None
}

pub async fn etcd_add_role(role: String)  -> Result<(),etcd_client::Error> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response = etcd.add_role(role).await;
         return response;
      }
   }
   Err(etcd_client::Error::InvalidArgs("".into()))
}

pub async fn etcd_del_role(role: String)  -> Result<(),etcd_client::Error> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response = etcd.delete_role(role).await;
         return response;
      }
   }
   Err(etcd_client::Error::InvalidArgs("".into()))
}

pub async fn etcd_grant_role_permissions(role: String,ty :String,path :String)  -> Result<(),etcd_client::Error> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   let mut _ty = etcd_client::PermissionType::Read;
   if ty == "write" {
      _ty = etcd_client::PermissionType::Write;
   } else if ty == "readwrite" {
      _ty = etcd_client::PermissionType::Write;
   }


   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response = etcd.garnt_role_permission(role,_ty,path).await;
         return response;
      }
   }
   Err(etcd_client::Error::InvalidArgs("".into()))
}

pub async fn etcd_revoke_role_permissions(role: String,ty :String,path :String,range_end: String)  -> Result<(),etcd_client::Error> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response = etcd.revoke_role_permission(role,path,range_end).await;
         return response;
      }
   }
   Err(etcd_client::Error::InvalidArgs("".into()))
}

pub async fn etcd_user_list()  -> Option<Vec<String>> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response =  etcd.user_lists().await;

         match response {
            Some(result) => {
               return Some(result);
            }
            _ => {

            }
         }
      }
   }
   None
}

pub async fn etcd_user_role_list(name :String)  -> Option<Vec<String>> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response =  etcd.user_get(name).await;

         match response {
            Some(result) => {
               return Some(result);
            }
            _ => {

            }
         }
      }
   }
   None
}

pub async fn etcd_user_add(name :String,password :String)  -> Result<(), etcd_client::Error> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response =  etcd.user_add(name,password).await;

         match response {
            Ok(result) => {
               return Ok(());
            }
            _ => {

            }
         }
      }
   }
   Err(etcd_client::Error::InvalidArgs("".into()))
}

pub async fn etcd_user_delete(name :String)  -> Result<(), etcd_client::Error> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response =  etcd.user_delete(name).await;
         match response {
            Ok(result) => {
               return Ok(());
            }
            _ => {

            }
         }
      }
   }
   Err(etcd_client::Error::InvalidArgs("".into()))
}

pub async fn user_grant_role(user :String,role :String)  -> Result<(), etcd_client::Error> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response =  etcd.user_grant_role(user,role).await;
         match response {
            Ok(result) => {
               return Ok(());
            }
            _ => {

            }
         }
      }
   }
   Err(etcd_client::Error::InvalidArgs("".into()))
}

pub async fn user_revoke_role(user :String,role :String)  -> Result<(), etcd_client::Error> {
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();

   if let Some(config) = urls {
      let state = etcd.open(config).await;
      if let Ok(true) = state {
         let response =  etcd.user_revoke_role(user,role).await;
         match response {
            Ok(result) => {
               return Ok(());
            }
            _ => {

            }
         }
      }
   }
   Err(etcd_client::Error::InvalidArgs("".into()))
}

#[cfg(test)]
mod test {
   use std::net::IpAddr::V4;
   use super::*;

   #[tokio::test]
   async fn test_etcd_get_all(){
     println!("hello");
     let result =  etcd_get_all("".to_owned()).await;

     if let Ok(records) = result {
        for kv in records {
            println!("{} = {}",kv.key,kv.value);
        }
     }
     println!("hello");
   }

   #[test]
   fn test_get_etcd_instance(){
      let config  = get_etcd_instance();
      println!("{:?}",config.unwrap().address);
   }
}

