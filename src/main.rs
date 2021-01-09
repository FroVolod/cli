use structopt::StructOpt;

mod login_command;
mod state_command;
mod working_with_keys_command;
mod construct_transaction_command;


#[derive(Debug, StructOpt)]
enum CliCommand {
    Login(login_command::Login),
    State(state_command::State),
    ConstructTransactionCommand(construct_transaction_command::TransactionCommand),
    WorkingWithKeys(working_with_keys_command::KeysCommand),
    
}

#[paw::main]
fn main(args: CliCommand) {
    println!("Hello, world! \n {:#?}", &args);
    match args {
        CliCommand::Login(login) => {
            login_command::Login::process(login);
        },
        CliCommand::ConstructTransactionCommand(transaction_command) => {
            construct_transaction_command::TransactionCommand::call(transaction_command);
        },
        CliCommand::WorkingWithKeys(keys_command) => {
            working_with_keys_command::KeysCommand::call(keys_command);
        }
        _ => println!("*** NONONO")
    }
}
