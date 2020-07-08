use bank::{Bank, BankAccount};
use ofx::Ofx;

pub struct CharlesSchwab {
    name: String,
    username: String,
    password: String,
    accounts: Option<Vec<BankAccount>>,
}

impl CharlesSchwab {
    pub fn new(name: String, username: String, password: String, accounts: Option<Vec<BankAccount>>) -> CharlesSchwab {
        CharlesSchwab {
            name: name,
            username: username,
            password: password,
            accounts: accounts,
        }
    }
}

impl Bank for CharlesSchwab {
    fn kind(&self) -> &str {
        "charles_schwab"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_ofx<'a>(&'a self) -> Option<&'a dyn Ofx> {
        Some(self as &dyn Ofx)
    }

    fn accounts(&self) -> Result<Vec<BankAccount>, String> {
        if let Some(ref accounts) = self.accounts {
            Ok(accounts.clone())
        } else {
            self.ofx_accounts()
        }
    }
}

impl Ofx for CharlesSchwab {
    fn url(&self) -> &str {
        "https://ofx.schwab.com/cgi_dev/ofx_server"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "1234"
    }

    fn fid_org(&self) -> &str {
        "SchwabRPS"
    }

    fn broker_id(&self) -> &str {
        "SchwabRPS.dv"
    }
}
