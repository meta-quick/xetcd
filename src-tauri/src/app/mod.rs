use crate::service::db::storage::Keydb;
use crate::service::etcd;
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use redb::Error;

#[derive(Deserialize,Serialize,Debug,Clone)]
struct EtcdConfig {
   pub name: String,
   pub address: String,
}

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

   if let Some(etcd_urls) = urls {
      let state = etcd.open(etcd_urls.address).await;

      if let Ok(true) = state {
         let kv = etcd::SimpleKeyValue { key, value};
         let response =  etcd.put(kv).await;
         match response {
             Ok(_) => {},
             Err(_) => {},
         }
      }
   } 
}

pub async fn etcd_get(key: String) -> Result<etcd::JSonKeyValue,Error>{
   let mut etcd = etcd::client::Etcd::new();
   let urls = get_etcd_instance();
   if let Some(etcd_urls) = urls {

      let state = etcd.open(etcd_urls.address).await;

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

pub async fn etcd_get_all(key: String) -> Result<Vec<etcd::JSonKeyValue>,Error>{
   let mut etcd = etcd::client::Etcd::new();

   let urls = get_etcd_instance();
   if let Some(etcd_urls) = urls {
      let state = etcd.open(etcd_urls.address).await;

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

   if let Some(etcd_urls) = urls {
      let state = etcd.open(etcd_urls.address).await;
      if let Ok(true) = state {
         let kv = etcd::SimpleKeyValue { key: key, value: "".to_string() };
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

#[cfg(test)]
mod test {
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

