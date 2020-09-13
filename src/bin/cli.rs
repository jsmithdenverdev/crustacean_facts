use clap::arg_enum;
use crustacean_facts::database;
use crustacean_facts::distribution;
use crustacean_facts::errors::Error;
use crustacean_facts::subscribers;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum Mode {
        List,
        Write,
        Delete
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, possible_values = &Mode::variants(), case_insensitive = true)]
    mode: Mode,
    #[structopt(name = "CONTACT")]
    contact: Option<String>,
}

fn run(opt: Opt) -> crustacean_facts::Result<()> {
    let store = database::SqliteSubscriberStore::new()?;
    let distributor = distribution::TwilioDistributor::new(
        String::from("AC88a6ebd3a388fe137ee4b731877e11a2"),
        String::from("561ed86f6c0bf27b0028185d8c9c6634"),
        String::from("+17324798571"),
    );
    // let distributor = distribution::MockDistributor {};
    let service = subscribers::SubscriberService::new(store, distributor);

    // match on the cli mode
    return match opt.mode {
        Mode::List => match service.list_subscribers()? {
            Some(subs) => {
                println!("Subscribers: {:?}", subs);
                Ok(())
            }
            None => {
                println!("No subscribers!");
                Ok(())
            }
        },
        Mode::Write => match opt.contact {
            Some(contact) => {
                service.create_subscriber(&contact)?;
                return Ok(());
            }
            None => return Err(Error(String::from("missing contact"))),
        },
        Mode::Delete => match opt.contact {
            Some(contact) => {
                service.delete_subscriber(&contact)?;
                return Ok(());
            }
            None => return Err(Error(String::from("missing contact"))),
        },
    };
}

fn main() {
    let opt = Opt::from_args();

    match run(opt) {
        Err(err) => println!("{:?}", err),
        Ok(()) => (),
    }
}
