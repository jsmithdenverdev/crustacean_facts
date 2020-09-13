use crate::errors::Error;
use crate::subscribers::{Contact, Subscriber, SubscriberStore};
use crate::Result;
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection};

pub struct SqliteSubscriberStore {
    conn: Connection,
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        // TODO: There is a better error conversion to be had here
        Error(error.to_string())
    }
}

impl SubscriberStore for SqliteSubscriberStore {
    fn read(&self, contact: &Contact) -> Result<Option<Subscriber>> {
        // prepare a query for the db
        let mut stmt = self
            .conn
            .prepare("select contact from subscriber where contact = ?")?;

        // execute the query
        let result = stmt.query_row(params![contact], |row| {
            let contact = row.get(0)?;
            return Ok(Subscriber { contact });
        });

        match result {
            Ok(subscriber) => Ok(Some(subscriber)),
            // check the error returned
            Err(variant) => match variant {
                // if the error was no rows then return an empty ok result
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                // otherwise convert the error into our error and return it
                _ => Err(Error::from(variant)),
            },
        }
    }

    fn write(&self, subscriber: &Subscriber) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("insert into subscriber(contact) values (?)")?;

        // execute the query
        stmt.execute(params![subscriber.contact])?;

        Ok(())
    }

    fn list(&self) -> Result<Option<Vec<Subscriber>>> {
        let mut stmt = self.conn.prepare("select contact from subscriber")?;

        // execute the query
        let subscriber_results = stmt.query_map(NO_PARAMS, |row| {
            let contact = row.get(0)?;
            Ok(Subscriber { contact })
        })?;

        let mut subscribers: Vec<Subscriber> = Vec::new();

        for subscriber_result in subscriber_results {
            let subscriber = subscriber_result?;
            subscribers.push(subscriber);
        }

        Ok(Some(subscribers))
    }

    fn delete(&self, contact: &Contact) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("delete from subscriber where contact = ?")?;

        // execute the query
        stmt.execute(params![contact])?;

        Ok(())
    }
}

impl SqliteSubscriberStore {
    pub fn new() -> Result<SqliteSubscriberStore> {
        let conn = Connection::open("subscribers.db");

        match conn {
            Err(_) => {
                return Err(Error(String::from("failed to open connection to database")));
            }
            Ok(conn) => {
                // initialize database
                if let Err(_) = conn.execute(
                    "CREATE TABLE IF NOT EXISTS subscriber (
                      id      INTEGER PRIMARY KEY,
                      contact TEXT NOT NULL
              )",
                    params![],
                ) {
                    return Err(Error(String::from("could not initialize database")));
                } else {
                    return Ok(SqliteSubscriberStore { conn });
                }
            }
        }
    }
}
