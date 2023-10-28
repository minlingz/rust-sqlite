use std::env;
use std::error::Error;
use rustsql::load;
use rustsql::extract;
use rustsql::query;
use rustsql::insert;
use rustsql::update;
use rustsql::delete;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let phase = &args[1];
    if phase == "load" {
    load("dataset/listings.csv")?;
    
    println!("Data loaded into airbnb.db");
    }
    else if phase == "extract" {
    let url = "https://anlane611.github.io/ids702-fall23/DAA/listings.csv";
    let _file_path = "dataset/listings.csv";
    let result = extract(url, None)?;
    println!("Data extracted to {}", result);
    }
    else if phase == "query" {
    let limit = 5;
    query(limit)?;
    }
    else if phase == "insert"{
    insert()?;
    println!("Data inserted into airbnb.db"); 
    }
    else if phase == "update" {
    update()?;
    println!("Data updated in airbnb.db");
    }
    else if phase == "delete" {
    delete()?;
    println!("Data deleted from airbnb.db");
        
    }
    else {
        println!("Please enter a valid phase: load or extract");
        
    }
    Ok(())
}
