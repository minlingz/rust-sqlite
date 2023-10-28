extern crate rusqlite;
extern crate csv;

use std::error::Error;
use std::fs::File;
use rusqlite::Connection;
use csv::ReaderBuilder;
use rusqlite::ToSql;
use reqwest;
use rusqlite::{params, Result};


pub fn extract(url: &str, file_path: Option<&str>) -> Result<String, Box<dyn Error>> {
    // download dataset from url and save it to file_path
    let _file_path = file_path.unwrap_or("dataset/listings.csv");
    let mut response = reqwest::blocking::get(url)?;
    let mut file = File::create(_file_path)?;
    response.copy_to(&mut file)?;
    Ok(_file_path.to_string())
}

pub fn load(dataset: &str) -> Result<(), Box<dyn Error>> {
    // load dataset into sqlite database
    let file = File::open(dataset)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let _headers = rdr.headers()?.clone();
    
    const TABLE_NAME: &str = "airbnb";
    let conn = Connection::open("dataset/airbnb.db")?;
    // drop table if exists
    conn.execute::<&[&dyn ToSql; 0]>(&format!("DROP TABLE IF EXISTS {}", TABLE_NAME), &[])?;    
    let headers: Vec<&str> = rdr.headers()?.iter().collect();
    // create table
    let create_table = format!("CREATE TABLE airbnb ({})", headers.join(", "));
    conn.execute::<&[&dyn ToSql]>(&create_table, &[])?;
    
    let mut stmt = conn.prepare(&format!("INSERT INTO airbnb VALUES ({})",
                                         headers.iter().map(|_| "?").collect::<Vec<_>>().join(", ")))
                                         .unwrap();

    for result in rdr.records() {
        let record = result?;
        let mut sanitized_record: Vec<&str> = record.iter().map(|field| field.trim_matches('"')).collect();
        // remove $ from price column
        sanitized_record[40] = sanitized_record[40].trim_start_matches('$');
        stmt.execute(&sanitized_record).unwrap();
    }

    Ok(())
}

pub fn insert() -> Result<(), Box<dyn Error>> {
    // insert a new record in the database
    let conn = Connection::open("dataset/airbnb.db")?;
    conn.execute(
        "INSERT INTO airbnb (neighbourhood, price) VALUES (?1, ?2)",
        params!["Dummy", "10000"],
    )?;
    Ok(())
}

pub fn update() -> Result<(), Box<dyn Error>> {
    // update a record in the database
    let conn = Connection::open("dataset/airbnb.db")?;
    conn.execute(
        "UPDATE airbnb SET price = ?1 WHERE neighbourhood = ?2",
        params!["20000", "Dummy"],
    )?;
    Ok(())
}

pub fn delete() -> Result<(), Box<dyn Error>> {
    // delete a record from the database
    let conn = Connection::open("dataset/airbnb.db")?;
    conn.execute("DELETE FROM airbnb WHERE neighbourhood = ?1", params!["Dummy"])?;
    Ok(())
}

pub fn query(limit: i64) -> Result<()> {
    //Read the database and return the average price per night for each neighbourhood
    let conn = Connection::open("dataset/airbnb.db")?;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM airbnb", params![], |row| row.get(0))?;
    println!("Number of rows in the airbnb table: {}", count);
    let mut stmt = conn.prepare(
        "SELECT neighbourhood, AVG(CAST(price AS REAL)) AS avg_price_per_night \
         FROM airbnb \
         GROUP BY neighbourhood \
         ORDER BY avg_price_per_night DESC \
         LIMIT ?",
    )?;
    let rows = stmt.query_map(params![limit], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    })?;
    for row in rows {
        let (neighbourhood, avg_price_per_night) = row?;
        println!("{}: ${:.2}/night", neighbourhood, avg_price_per_night);
    }
    Ok(())
}

