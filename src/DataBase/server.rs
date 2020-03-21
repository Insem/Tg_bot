use mongodb::{Client, options::ClientOptions, options::StreamAddress};
use std::error::Error;
use core::result::Result;
use bson::{doc, bson};
#[derive(Debug)]
pub struct  DB{
    db:Result<Client, Box<dyn Error>>
    //создать перечисление баз данных, с типажом
}

impl DB  {
    pub fn start() -> DB{
        let options = ClientOptions::builder()
                  .hosts(vec![
                      StreamAddress {
                          hostname: "localhost".into(),//чекнуть функцию
                          port: Some(27017),
                      }
                  ])
                  .build();
        DB{
           db:Ok(Client::with_options(options).expect("Connection to DB failed"))
        }
        
    }

    pub fn insert(&self){
        let db = self.db.as_ref().unwrap().database("test");

        let collection = db.collection("music");

        let docs = vec![
            doc! { "tg_id":"040053053045" },
            doc! { "tg_id":"040053053045" },
            doc! { "tg_id":"040053053045" },
        ];
        collection.insert_many(docs, None).unwrap();
    }
}