use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CliArgs {
    #[structopt(subcommand)]
    subcommand: CliCommand,
}

#[derive(Debug, StructOpt)]
enum CliCommand {
    Login,
    CreateAccount,
    State,
    Keys,
    AddKey,
    DeleteKey,
    GenerateKey,
    Send,
    Stake,
    Delete,
}

#[paw::main]
fn main(args: CliArgs) {
    println!("Hello, world! {:?}", args.subcommand);
}
