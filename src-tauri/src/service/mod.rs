pub(crate) mod storage{
    use std::path::PathBuf;
    use std::fs;
    use redb::{Database, Error, TableDefinition, ReadableTable};

    const ETCD: TableDefinition<&str, &str> = TableDefinition::new("etcd");
    pub struct Keydb{
       db: Database,
       state: bool
    }

    impl Keydb {
        pub fn open() -> Keydb{
           let db = open_db().unwrap();
           let inst = Keydb {
              db,
              state: true
           };

           return inst;
        }

        pub fn put(&self, key: &str, val: &str) -> Result<(),Error>{
            let write_txn = self.db.begin_write()?;
            {
                let mut table = write_txn.open_table(ETCD)?;
                table.insert(key,val)?;
            }
            write_txn.commit()?;

            Ok(())
        }

        pub fn remove(&self, key: &str) -> Result<(),Error>{
            let write_txn = self.db.begin_write()?;
            {
                let mut table = write_txn.open_table(ETCD)?;
                table.remove(key).expect("not exist");
            }
            write_txn.commit()?;

            Ok(())
        }

        pub fn get(&self, key: &str) -> Result<String,Error>{
            let read_txn = self.db.begin_read()?;
            let table = read_txn.open_table(ETCD)?;
            
            let resultset = table.get(key);

            match resultset {
                Ok(val) => {
                   match val {
                     Some(records) => {
                        return Ok(records.value().clone().to_owned());
                     },
                     None => {

                     },
                   }
                },
                Err(_) => {

                },
            }
            return Ok("Err".to_owned());
        }
    }


    pub fn db_file_name() -> Option<PathBuf>{
        let dbpath = ".xetcd";
        let mut fulldbpath = None;
        let dbfile = "data.re";

        match home::home_dir() {
            Some(path) => {
                let path = path.join(dbpath);
                match &path.try_exists() {
                  Ok(true) => {},
                  Ok(false) => {
                    match fs::create_dir(&path.as_path()) {
                        Ok(()) => println!("Directory created: {:?}", &path.as_path()),
                        Err(err) => eprintln!("Error creating directory: {}", err),
                    }
                  },
                  Err(_) => {panic!("cannot trave home directory")}
                }
                fulldbpath = Some(path);
            },
            None => panic!("Impossible to get your home dir!"),
        };

        //Here the target dir already created, then create db if not exist
        if fulldbpath.is_some() {
           let db = fulldbpath.unwrap().join(dbfile);
           match db.try_exists(){
            Ok(true) => {
                return Some(db.clone());
            },
            Ok(false) => {
                match fs::File::create(db.as_path()) {
                    Ok(_file) => {
                        return Some(db.clone());
                    },
                    Err(err) => eprintln!("Error creating directory: {}", err),
                }
            },
            Err(_) => {}
           }
        }
        None
    }

    pub fn open_db() -> Result<Database,Error>{
       let file = db_file_name().unwrap();
       let db = Database::create(file.as_path())?;
       return Ok(db);
    }
   
}

#[cfg(test)]
mod test {
    //redb testcase
    use super::storage::*;
    use redb::{Database, Error, ReadableTable, TableDefinition};
    const TABLE: TableDefinition<&str, u64> = TableDefinition::new("my_data");

   #[test]
   pub fn test_create_db(){
        db_file_name();
   }

   #[test]
   pub fn test_crud_db() -> Result<(), Error>{
        let db = open_db().unwrap();
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            table.insert("my_key", &123)?;
        }
        write_txn.commit()?;
    
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;
        assert_eq!(table.get("my_key")?.unwrap().value(), 123);
    
        Ok(())
   }

    #[test]
    pub fn testredb() -> Result<(), Error>{
        let file = tempfile::NamedTempFile::new().unwrap();
        let db = Database::create(file.path())?;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            table.insert("my_key", &123)?;
        }
        write_txn.commit()?;
    
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;
        assert_eq!(table.get("my_key")?.unwrap().value(), 123);
    
        Ok(())
    }
}



