use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum CliCommand {
    Login(Login),
    CreateAccount(CreateAccount),
    State(State),
    Keys(Keys),
    AddKey(AddKey),
    DeleteKey(DeleteKey),
    GenerateKey(GenerateKey),
    Send(Send),
    Stake(Stake),
    DeleteAccount(DeleteAccount),
}

#[derive(Debug, StructOpt)]
struct Login {
    account_id: String,
}

#[derive(Debug, StructOpt)]
struct CreateAccount {
    account_id: String,
    }

#[derive(Debug, StructOpt)]
struct State {
    account_id: String,
    }

#[derive(Debug, StructOpt)]
struct Keys {
    account_id: String,
    }

#[derive(Debug, StructOpt)]
struct AddKey {
    account_id: String,
    }

#[derive(Debug, StructOpt)]
struct DeleteKey {
    account_id: String,
    }

#[derive(Debug, StructOpt)]
struct GenerateKey {
    account_id: String,
    }

#[derive(Debug, StructOpt)]
struct Send {
    #[structopt(long)]  // Можно удалить строку. Тогда субкоманда будет работать без "флага".
    account_id: String,
    #[structopt(long)]
    receiver_id: String,
    #[structopt(long)]
    amount: u128,
}

#[derive(Debug, StructOpt)]
struct Stake {
    account_id: String,
    amount: u128,
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
