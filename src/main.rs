use structopt::StructOpt;

mod login_command;
mod create_account_command;
mod state_command;
mod keys_command;
mod add_key_command;
mod delete_key_command;
mod generate_key_command;
mod send_command;
mod stake_command;
mod delete_account_command;


#[derive(Debug, StructOpt)]
enum CliCommand {
    Login(login_command::Login),
    CreateAccount(create_account_command::CreateAccount),
    State(state_command::State),
    Keys(keys_command::Keys),
    AddKey(add_key_command::AddKey),
    DeleteKey(delete_key_command::DeleteKey),
    GenerateKey(generate_key_command::GenerateKey),
    Send(send_command::Send),
    Stake(stake_command::Stake),
    DeleteAccount(delete_account_command::DeleteAccount),
}

#[derive(Debug, StructOpt)]
struct DeleteAccount {
    account_id: String,
}

#[paw::main]
fn main(args: CliCommand) {
    println!("Hello, world! \n {:#?}", &args);
    match args {
        CliCommand::Login(Login) => println!("*** login"),
        CliCommand::CreateAccount(CreateAccount) => println!("*** create account"),
        CliCommand::State(State) => println!("*** state"),
        CliCommand::Keys(Keys) => println!("*** keys"),
        CliCommand::AddKey(AddKey) => println!("*** add key"),
        CliCommand::GenerateKey(GenerateKey) => println!("*** generate key"),
        CliCommand::DeleteKey(DeleteKey) => println!("*** delete key"),
        CliCommand::Send(Send) => println!("*** send"),
        CliCommand::Stake(Stake) => println!("*** stake"),
        CliCommand::DeleteAccount(DeleteAccount) => println!("*** delete account"),
        // _ => println!("*** NONONO")
    }
}
