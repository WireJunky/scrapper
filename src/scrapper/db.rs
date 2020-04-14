use crate::scrapper::scrape;

use rusqlite::{params, Connection, Transaction, Result};

pub fn initialize(dbpath:&str)-> Result<()>{
    let conn = Connection::open(dbpath)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS Business (
            id	INTEGER,
            name	TEXT,
            address	TEXT,
            address2	BLOB,
            street	BLOB,
            city	TEXT,
            postal_code	TEXT,
            state	TEXT,
            country	TEXT,
            latitude	TEXT,
            longitude	TEXT,
            PRIMARY KEY(id)
        );
        
        CREATE TABLE IF NOT EXISTS Email (
            id	INTEGER,
            business_id	INTEGER,
            value	TEXT,
            PRIMARY KEY(id),
            FOREIGN KEY(business_id) REFERENCES Business(id)
        );
        
        CREATE TABLE IF NOT EXISTS Telephone (
            id	INTEGER,
            business_id	INTEGER,
            value	TEXT,
            PRIMARY KEY(id),
            FOREIGN KEY(business_id) REFERENCES Business(id)
        );
        
        DELETE FROM Telephone;
        DELETE FROM Email;
        DELETE FROM Business;   
        "
    )?;
    
    Ok(())
}

fn add_emails(trans:&Transaction, business_id:i64, emails:&Vec<String>) -> Result<()>{  
    for mail in emails {
        trans.execute(
            "INSERT INTO Email (business_id, value) VALUES (?1, ?2)",
            params![business_id, mail],
        )?;
    }
    Ok(())
}

fn add_phone_numbers(trans:&Transaction, business_id:i64, phone_numbers:&Vec<String>) -> Result<()>{  
    for number in phone_numbers {
        trans.execute(
            "INSERT INTO Telephone (business_id, value) VALUES (?1, ?2)",
            params![business_id, number],
        )?;
    }
    Ok(())
}

fn add_businesses(trans:&Transaction, businesses:&Vec<scrape::BusinessRecord>) -> Result<()>{
    for b in businesses {
        trans.execute(
            "INSERT INTO Business(name,address,address2,street,city,postal_code,state,country,latitude,longitude) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);",
            params![b.name, b.address, b.address2, b.street, b.city, b.postal_code, b.state, b.country, b.latitude, b.longitude],
        )?;
        let business_id = trans.last_insert_rowid();
        add_emails(&trans, business_id, &b.email)?;
        add_phone_numbers(&trans, business_id, &b.telephone)?;
    }
    Ok(())
}


pub fn store_businesses(dbpath:&str, businesses:&Vec<scrape::BusinessRecord>)-> Result<()> {
    let mut conn = Connection::open(dbpath)?;
    let tx = conn.transaction()?;
    add_businesses(&tx, businesses)?;
    tx.commit()?;
    Ok(())
}