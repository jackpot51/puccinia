use decimal::d128;

use bank::{Bank, BankAccount};
use ofx::Ofx;

pub struct Vanguard {
    name: String,
    username: String,
    password: String,
    accounts: Option<Vec<BankAccount>>,
}

impl Vanguard {
    pub fn new(name: String, username: String, password: String, accounts: Option<Vec<BankAccount>>) -> Vanguard {
        Vanguard {
            name: name,
            username: username,
            password: password,
            accounts: accounts,
        }
    }
}

impl Bank for Vanguard {
    fn kind(&self) -> &str {
        "vanguard"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn accounts(&self) -> Result<Vec<BankAccount>, String> {
        if let Some(ref accounts) = self.accounts {
            Ok(accounts.clone())
        } else {
            self.ofx_accounts()
        }
    }

    fn amount(&self, account: &BankAccount) -> Result<d128, String> {
        self.ofx_amount(&account.id, &account.kind)
    }

    fn as_ofx<'a>(&'a self) -> Option<&'a Ofx> {
        Some(self as &Ofx)
    }
}

impl Ofx for Vanguard {
    fn url(&self) -> &str {
        "https://vesnc.vanguard.com/us/OfxDirectConnectServlet"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "1358"
    }

    fn fid_org(&self) -> &str {
        "Vanguard"
    }

    fn broker_id(&self) -> &str {
        "vanguard.com"
    }
}
