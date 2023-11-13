use serde::{Serialize, Deserialize};
use crate::types::dto::*;
pub mod client{
    use std::ops::Add;
    use etcd_client::*;
    use super::*;

    // #[derive(Serialize, Deserialize,Clone)]
    pub struct Etcd{
        client: Option<Client>,
        state: bool
    }

     impl Etcd {
        pub fn new() -> Etcd{
            Etcd { client: None, state: false }
        }
        pub async fn open(&mut self,config: EtcdConfig) ->Result<bool,Error>{
            let mut etcd_config = config.clone();
            if !&etcd_config.address.contains(";"){
                etcd_config.address = etcd_config.address.add(";");
            }

            let urls :Vec<&str> =  etcd_config.address.split(';').filter(|addr| !addr.is_empty()).collect();

            let mut option = None;
            if config.authway.eq("password"){
                let user = config.username.unwrap();
                let password = config.password.unwrap();
                let passwordOption = etcd_client::ConnectOptions::new().with_user(user,password);
                option = Some(passwordOption);
            }

            let client = Client::connect(&urls[..], option).await;
            match client {
                Ok(cli) => {
                    self.client = Some(cli);
                    self.state = true;
                }
                Err(err) => {
                    return Err(Error::from(err));
                }
            }
            Ok(true)
        }

        pub async fn put(&mut self,kv: SimpleKeyValue) ->Result<(),Error>{
            if self.state {
                let cli = self.client.as_mut().unwrap();
                cli.put(kv.key(), kv.value(), None).await?;
            }
            Ok(())
        }

        pub async fn put_any(&mut self,kv: AnyKeyValue) ->Result<(),Error>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let val = kv.value().as_str();
                 match val {
                     Some(value) => {
                         cli.put(kv.key(), kv.value().as_str().unwrap(), None).await?;
                     }
                     _ => {
                         cli.put(kv.key(), kv.value().to_string(), None).await?;
                     }
                 }


             }
             Ok(())
        }

        pub async fn get(&mut self,key: String) ->Result<JSonKeyValue,Error>{
            if self.state {
                let cli = self.client.as_mut().unwrap();
                let resp = cli.get(key, None).await?;
                if let Some(kv) = resp.kvs().first() {
                   let json_kv = JSonKeyValue{
                      key: kv.key_str()?.into(),
                      value: kv.value_str()?.into(),
                      create_revision: kv.create_revision(),
                      mod_revision: kv.mod_revision(),
                      version: kv.version(),
                      lease: kv.lease(),
                   };
                   return Ok(json_kv);
                }
            }
            return Err(client::Error::InvalidArgs("".into()));
        }

        pub async fn get_all(&mut self,key: String) ->Result<Vec<JSonKeyValue>,Error>{
            if self.state {
                let cli = self.client.as_mut().unwrap();
                let resp = cli.get(key, Some(GetOptions::new().with_all_keys())).await?;
                
                let mut results = Vec::<JSonKeyValue>::new();

                for kv in resp.kvs() {
                    let json_kv = JSonKeyValue{
                        key: kv.key_str()?.into(),
                        value: kv.value_str()?.into(),
                        create_revision: kv.create_revision(),
                        mod_revision: kv.mod_revision(),
                        version: kv.version(),
                        lease: kv.lease(),
                     };
                    
                    results.push(json_kv);
                }
                return Ok(results);
            }
            return Err(client::Error::InvalidArgs("".into()));
        }

        pub async fn remove(&mut self,kv: SimpleKeyValue) ->Result<(),Error>{
            if self.state {
                let cli = self.client.as_mut().unwrap();
                let options =  DeleteOptions::new().with_prefix();
                let response = cli.delete(kv.key(), Some(options)).await;
                match response {
                    Ok(_result) => {
                       return Ok(());
                    },
                    Err(_err) => {
                        return Err(client::Error::InvalidArgs("".into()));
                    }
                }
            }
            Ok(())
        }

        pub async fn list_roles(&mut self) -> Option<Vec<String>>{
            if self.state {
                let cli = self.client.as_mut().unwrap();
                let response = cli.role_list().await;
                match response {
                    Ok(result) => {
                        let list :Vec<String> =  result.roles().iter().map(|item|{
                           let a = item.as_str();
                           return a.to_string();
                        }).collect();
                       return Some(list);
                    }
                    _ => {
                        return None;
                    }
                }
            }
            None
        }

         pub async fn delete_role(&mut self,role :String) -> Result<(),Error>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let response = cli.role_delete(role).await;
                 match response {
                     Ok(_result) => {
                         return Ok(());
                     }
                     _ => {
                         return Err(Error::InvalidArgs("".into()));
                     }
                 }
             }
             return Err(Error::InvalidArgs("".into()));
         }

         pub async fn add_role(&mut self,role :String) -> Result<(),Error>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let response = cli.role_add(role).await;
                 match response {
                     Ok(_result) => {
                         return Ok(());
                     }
                     _ => {
                         return Err(Error::InvalidArgs("".into()));
                     }
                 }
             }
             return Err(Error::InvalidArgs("".into()));
         }

         pub async fn get_role_permission(&mut self,role :String) -> Option<Vec<Permission>>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let response = cli.role_get(role).await;
                 match response {
                     Ok(result) => {
                         return Some(result.permissions());
                     }
                     _ => {
                         return None;
                     }
                 }
             }
             return None;
         }

         pub async fn revoke_role_permission(&mut self,role :String,key: String,range_end: String) -> Result<(),Error>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let option = RoleRevokePermissionOptions::new().with_prefix().with_range_end(range_end);
                 let response = cli.role_revoke_permission(role,key,Some(option)).await;
                 match response {
                     Ok(_result) => {
                         return Ok(());
                     }
                     _ => {
                         return Err(Error::InvalidArgs("".into()));
                     }
                 }
             }
             return Err(Error::InvalidArgs("".into()));
         }

         pub async fn garnt_role_permission(&mut self,role :String,ty :etcd_client::PermissionType,key: String) -> Result<(),Error>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();

                 let permission = Permission::new(ty,key).with_prefix();
                 let response = cli.role_grant_permission(role,permission).await;
                 match response {
                     Ok(_result) => {
                         return Ok(());
                     }
                     _ => {
                         return Err(Error::InvalidArgs("".into()));
                     }
                 }
             }
             return Err(Error::InvalidArgs("".into()));
         }

         pub async fn user_lists(&mut self) -> Option<Vec<String>>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let response = cli.user_list().await;
                 match response {
                     Ok(result) => {
                         let list :Vec<String> =  result.users().iter().map(|item|{
                             let a = item.as_str();
                             return a.to_string();
                         }).collect();

                         return  Some(list);
                     }
                     _ => {
                         return None;
                     }
                 }
             }
             return None;
         }

         pub async fn user_get(&mut self,name :String) -> Option<Vec<String>>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let response = cli.user_get(name).await;
                 match response {
                     Ok(result) => {
                         let list :Vec<String> =  result.roles().iter().map(|item|{
                             let a = item.as_str();
                             return a.to_string();
                         }).collect();

                         return  Some(list);
                     }
                     _ => {
                         return None;
                     }
                 }
             }
             return None;
         }

         pub async fn user_add(&mut self,name :String, password :String) -> Result<(),etcd_client::Error>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let option = etcd_client::UserAddOptions::new();
                 let response = cli.user_add(name,password,Some(option)).await;
                 match response {
                     Ok(result) => {
                         return  Ok(());
                     }
                     _ => {
                         return Err(etcd_client::Error::InvalidArgs("".into()));
                     }
                 }
             }
             return Err(etcd_client::Error::InvalidArgs("".into()));
         }

         pub async fn user_delete(&mut self,name :String) -> Result<(),etcd_client::Error>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let response = cli.user_delete(name).await;
                 match response {
                     Ok(result) => {
                         return  Ok(());
                     }
                     _ => {
                         return Err(etcd_client::Error::InvalidArgs("".into()));
                     }
                 }
             }
             return Err(etcd_client::Error::InvalidArgs("".into()));
         }

         pub async fn user_grant_role(&mut self,user :String,role :String) -> Result<(),etcd_client::Error>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let response = cli.user_grant_role(user,role).await;
                 match response {
                     Ok(result) => {
                         return  Ok(());
                     }
                     _ => {
                         return Err(etcd_client::Error::InvalidArgs("".into()));
                     }
                 }
             }
             return Err(etcd_client::Error::InvalidArgs("".into()));
         }

         pub async fn user_revoke_role(&mut self,user :String,role :String) -> Result<(),etcd_client::Error>{
             if self.state {
                 let cli = self.client.as_mut().unwrap();
                 let response = cli.user_revoke_role(user,role).await;
                 match response {
                     Ok(result) => {
                         return  Ok(());
                     }
                     _ => {
                         return Err(etcd_client::Error::InvalidArgs("".into()));
                     }
                 }
             }
             return Err(etcd_client::Error::InvalidArgs("".into()));
         }
     }
}


#[cfg(test)]
mod test {
    use super::*;
    use etcd_client::*;

    #[tokio::test]
    pub async fn test_open_user(){
        let mut option = None;
        let user = "root";
        let password = "zaq12wsx";
        let passwordOption = etcd_client::ConnectOptions::new().with_user(user,password);
        option = Some(passwordOption);

        let client = Client::connect(["192.168.11.155:2379"], option).await;
        match client {
            Ok(cli) => {
                println!("{:?}","cli");
            },
            Err(err) => {
                println!("{:?}",err);
            }
        }
    }

    #[tokio::test]
    pub async fn test_rule(){
        let mut option = None;
        let user = "root";
        let password = "zaq12wsx";
        let passwordOption = etcd_client::ConnectOptions::new().with_user(user,password);
        option = Some(passwordOption);

        let client = Client::connect(["192.168.11.155:2379"], option).await;
        match &client {
            Ok(cli) => {
                println!("{:?}","cli");
            },
            Err(err) => {
                println!("{:?}",err);
            }
        }

        let resp = client.unwrap().role_list().await;
        match resp {
            Ok(res) => {
                println!("Roles: \n {:?} ", res.roles());
            }
            _ => {}
        }
    }
}