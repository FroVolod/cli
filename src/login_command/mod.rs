use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Login {
    account_id: String,
}
