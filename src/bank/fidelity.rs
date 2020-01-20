use bank::{Bank, BankAccount};
use ofx::Ofx;

pub struct Fidelity {
    name: String,
    username: String,
    password: String,
    accounts: Option<Vec<BankAccount>>,
}

impl Fidelity {
    pub fn new(name: String, username: String, password: String, accounts: Option<Vec<BankAccount>>) -> Fidelity {
        Fidelity {
            name: name,
            username: username,
            password: password,
            accounts: accounts,
        }
    }
}

impl Bank for Fidelity {
    fn kind(&self) -> &str {
        "fidelity"
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

impl Ofx for Fidelity {
    fn url(&self) -> &str {
        "https://ofx.fidelity.com/ftgw/OFX/clients/download"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "7776"
    }

    fn fid_org(&self) -> &str {
        "fidelity.com"
    }

    fn broker_id(&self) -> &str {
        "fidelity.com"
    }
}
