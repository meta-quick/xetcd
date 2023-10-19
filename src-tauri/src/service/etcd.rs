
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct JSonKeyValue {
    pub key: String,
    pub create_revision: i64,
    pub mod_revision: i64,
    pub version: i64,
    pub value: String,
    pub lease: i64,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct SimpleKeyValue {
    pub key: String,
    pub value: String,
}

impl SimpleKeyValue {
    #[inline]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        SimpleKeyValue {
            key: key.into(),
            value: value.into(),
        }
    }

    #[inline]
    pub fn key(&self) -> &str {
        &self.key
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    } 
}


impl JSonKeyValue {
    #[inline]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        JSonKeyValue {
            key: key.into(),
            value: value.into(),
            create_revision: 0,
            mod_revision: 0,
            version: 0,
            lease: 0
        }
    }

    #[inline]
    pub fn key(&self) -> &str {
        &self.key
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }
}

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
        pub async fn open(&mut self,address: String) ->Result<bool,Error>{
            let mut url = address.clone();
            if !&url.contains(";"){
                url = url.add(";");
            }

            let urls :Vec<&str> =  url.split(';').filter(|addr| !addr.is_empty()).collect();

            let client = Client::connect(&urls[..], None).await;
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
                    Ok(result) => {
                       return Ok(());
                    },
                    Err(err) => {
                        return Err(client::Error::InvalidArgs("".into()));
                    }
                }
            }
            Ok(())
        }
     }
}


#[cfg(test)]
mod test {
    use super::*;
    use etcd_client::Error;

    #[tokio::test]
    pub async fn test_create_db() ->Result<(),Error>{
        let mut etcd = client::Etcd::new();
        let state = etcd.open("192.168.11.214:30380".to_owned()).await;
        if let Ok(true) = state {
            let kv = SimpleKeyValue { key: "/test".to_string(), value: "".to_string() };
            let response =  etcd.remove(kv).await;
            match response {
                Ok(result) => {
                  return Ok(result)
               },
                Err(_) => { return Err(Error::InvalidArgs("".into()))},
            }
        }
        Ok(())
    }
}