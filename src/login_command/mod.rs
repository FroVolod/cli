use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Login {
    account_id: String,
}

impl Login {
    pub fn process(self) -> String {
        println!("********* this is module named \"login_command\":\n  {:?}", &self);
        self.account_id
    }
}

