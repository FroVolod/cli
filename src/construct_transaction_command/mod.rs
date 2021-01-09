use structopt::StructOpt;

mod create_account_command;
mod delete_account_command;
mod send_command;
mod stake_command;

#[derive(Debug, StructOpt)]
pub enum TransactionCommand {
    CreateAccount(create_account_command::CreateAccount),
    Send(send_command::Send),
    Stake(stake_command::Stake),
    DeleteAccount(delete_account_command::DeleteAccount),
}

impl TransactionCommand {
    pub fn call(self) {
        println!("#### this is module named \"construct_transaction_command\":\n {:#?}", &self)
    }
}
