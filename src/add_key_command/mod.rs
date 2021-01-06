use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct AddKey {
    account_id: String,
}
