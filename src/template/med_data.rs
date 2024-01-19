use std::{collections::HashMap, io::Cursor, fs::File};

use axum::{extract::Path, http::StatusCode, response::IntoResponse};

//use hyper::{Request as HyperRequest, Response};
use reqwest;
use polars::prelude::*;


pub async fn update_file(Path(disease_name): Path<String>) -> impl IntoResponse {
    let (url, file_name) = get_path(disease_name)?;
    let csv_path = format!("csv/{}",file_name);
    let csv_path = csv_path.as_str();
    //let response = reqwest::get("https://httpbin.org/ip")
    let Ok(response) = reqwest::get(url).await else {
        return Err(StatusCode::EXPECTATION_FAILED);
        };
    
    let Ok(mut csv_file) = std::fs::File::create(csv_path) else {
        return Err(StatusCode::EXPECTATION_FAILED);
        };
    let mut content = Cursor::new(response.bytes().await.unwrap());
    
    
    std::io::copy(&mut content, &mut csv_file).expect("writing failed");

    let mut df = CsvReader::from_path(csv_path).unwrap().finish().unwrap();
    //println!("{}", df);
    
    let ipc_path = format!("ipc/{}.arrow", file_name);
    let mut ipc_file = File::create(ipc_path.clone()).unwrap_or_else(|_| panic!("Could not create ipc file at {}", ipc_path));
    IpcWriter::new(&mut ipc_file).finish(&mut df).unwrap();
    Ok(StatusCode::OK)
}

fn get_path(disease_name: String) -> Result<(&'static str, &'static str), StatusCode> {
    let d_id: HashMap<_, _> = HashMap::from([
        (
            "sim".to_string(),
            "https://www.datos.gov.co/resource/tmet-yeek.csv",
        ),
        (
            "gripe".to_string(),
            "https://www.datos.gov.co/resource/sq8q-pnf5.csv",
        ),
    ]);

    let Some(url) = d_id.get(&disease_name) else {
        return Err(StatusCode::EXPECTATION_FAILED);
    };
    let url = *url;
    let path = url.split('/').last().unwrap();
    
    Ok((url, path))
}


// Disease ID
