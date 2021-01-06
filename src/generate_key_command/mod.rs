use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct GenerateKey {
    account_id: String,
}
