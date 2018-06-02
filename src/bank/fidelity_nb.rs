use bank::{Bank, BankAccount};
use ofx::Ofx;

pub struct FidelityNb {
    name: String,
    username: String,
    password: String,
    accounts: Option<Vec<BankAccount>>,
}

impl FidelityNb {
    pub fn new(name: String, username: String, password: String, accounts: Option<Vec<BankAccount>>) -> FidelityNb {
        FidelityNb {
            name: name,
            username: username,
            password: password,
            accounts: accounts,
        }
    }
}

impl Bank for FidelityNb {
    fn kind(&self) -> &str {
        "fidelity_nb"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_ofx<'a>(&'a self) -> Option<&'a Ofx> {
        Some(self as &Ofx)
    }

    fn accounts(&self) -> Result<Vec<BankAccount>, String> {
        if let Some(ref accounts) = self.accounts {
            Ok(accounts.clone())
        } else {
            self.ofx_accounts()
        }
    }
}

impl Ofx for FidelityNb {
    fn url(&self) -> &str {
        "https://nbofx.fidelity.com/netbenefits/ofx/download"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "8288"
    }

    fn fid_org(&self) -> &str {
        "nbofx.fidelity.com"
    }

    fn broker_id(&self) -> &str {
        "nbofx.fidelity.com"
    }
}
