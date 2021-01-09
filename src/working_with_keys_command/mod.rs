use structopt::StructOpt;

mod keys_command;
mod add_key_command;
mod delete_key_command;
mod generate_key_command;

#[derive(Debug, StructOpt)]
pub enum KeysCommand {
    Keys(keys_command::Keys),
    AddKey(add_key_command::AddKey),
    DeleteKey(delete_key_command::DeleteKey),
    GenerateKey(generate_key_command::GenerateKey),
}

impl KeysCommand {
    pub fn call(self) {
        println!("=== this is module named \"working_with_keys\":\n {:?}", &self)
    }
}
