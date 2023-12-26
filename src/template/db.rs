

use mongodb::{
    Client, Collection,
};
use bson;
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path;
use std::io::{self, BufRead};


pub async fn get_mongo_users() -> axum::response::Json<Vec<User>>{
    // Print the databases in our MongoDB cluster:
    // Hard coded BS
    // Load the MongoDB connection string from an environment variable:
    let client_uri = get_uri().expect("Read secrets failed");
    let cl = Client::with_uri_str(client_uri).await.expect("DB connection failed");
    let user_col : Collection<User> = cl.database("Medetrics").collection("usuarios");
    let cursor = user_col.find(None, None).await.expect("Retrieve users failed");
    let docs: Vec<_> = cursor.try_collect().await.unwrap();
    
    axum::response::Json(docs)
}

#[derive(Serialize, Deserialize)]
pub struct User {
    nombre: String,
    email:String,
    contraseña:String,
    fecha_de_nacimiento:bson::DateTime,
    numero_salud: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Reporte {
    disease: String,
    municipio:String,
    barrio:String,
    diagnosis_place:String,
    diagnosis_date:bson::DateTime,
    sexo:char,
    edad:u8,
    estrato:u8,
    fecha_sintomasñ:i32
}

fn get_uri() -> Option<String> {

    let Ok(mut lines) = read_lines("secret.txt") else {
        return None;
    };
    let Some(Ok(line)) = lines.next() else { return None};
    Some(line)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<path::Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}