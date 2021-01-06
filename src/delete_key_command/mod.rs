use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct DeleteKey {
    account_id: String,
}
