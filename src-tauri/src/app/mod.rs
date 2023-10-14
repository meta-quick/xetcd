use crate::service::storage::Keydb;

use redb::{Error};

pub fn save(key: String,val: String){
   let db = Keydb::open();
   db.put(&key,&val).unwrap();
}
pub fn query(key: String) -> Result<String,Error>{
   let db = Keydb::open();
   return db.get(&key);
}

pub fn remove(key: String){
   let db = Keydb::open();
   db.remove(&key).unwrap();
}


#[tauri::command]
pub fn save_cluster(cluster: &str){
    let key = "cluster/records";
    print!("{}",cluster);
    save(key.to_owned(),cluster.to_owned());
}



