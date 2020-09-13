use crate::distribution::Distributor;
use crate::subscribers::SubscriberStore;
use crate::Result;

type Fact = String;

pub trait FactRetriever {
    fn retrieve() -> Result<Fact>;
}

fn _send_fact_to_subscribers<R: FactRetriever, S: SubscriberStore, D: Distributor>(
    _retriever: &R,
    _store: &S,
    _distributor: &D,
) -> Result<()> {
    Ok(())
}
