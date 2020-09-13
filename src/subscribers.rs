use crate::distribution::Distributor;
use crate::errors::Error;
use crate::Result;

pub type Contact = String;

#[derive(Debug)]
pub struct Subscriber {
    pub contact: Contact,
}

pub trait SubscriberStore {
    fn read(&self, contact: &Contact) -> Result<Option<Subscriber>>;
    fn write(&self, subscriber: &Subscriber) -> Result<()>;
    fn list(&self) -> Result<Option<Vec<Subscriber>>>;
    fn delete(&self, contact: &Contact) -> Result<()>;
}

pub struct SubscriberService<S, D>
where
    S: SubscriberStore,
    D: Distributor,
{
    store: S,
    distributor: D,
}

impl<S, D> SubscriberService<S, D>
where
    S: SubscriberStore,
    D: Distributor,
{
    pub fn new(store: S, distributor: D) -> SubscriberService<S, D> {
        SubscriberService { store, distributor }
    }

    pub fn create_subscriber(&self, contact: &Contact) -> Result<()> {
        let existing = self.store.read(contact)?;

        if let Some(_) = existing {
            return Err(Error(String::from(
                "a subscriber with this contact already exists",
            )));
        }

        let subscriber = Subscriber {
            contact: contact.to_owned(),
        };

        self.store.write(&subscriber)?;

        self.distributor
            .distribute(contact, "Hello! Welcome to crustacean facts!")?;

        Ok(())
    }

    pub fn delete_subscriber(&self, contact: &Contact) -> Result<()> {
        let existing: Option<Subscriber> = self.store.read(contact)?;

        if let None = existing {
            return Err(Error(String::from("no subscriber exists for this contact")));
        }

        self.store.delete(contact)?;

        self.distributor
            .distribute(contact, "Goodbye! Farewell from crustacean facts!")?;

        Ok(())
    }

    pub fn list_subscribers(&self) -> Result<Option<Vec<Subscriber>>> {
        // this is just a passthrough to the data layer. allowing us to fetch
        // a list of subscribers without be tied to a specific provier.
        Ok(self.store.list()?)
    }
}
