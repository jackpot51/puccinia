use decimal::d128;

use bank::{Bank, BankAccount};
use ofx::Ofx;

pub struct Tangerine {
    name: String,
    username: String,
    password: String,
    accounts: Option<Vec<BankAccount>>,
}

impl Tangerine {
    pub fn new(name: String, username: String, password: String, accounts: Option<Vec<BankAccount>>) -> Tangerine {
        Tangerine {
            name: name,
            username: username,
            password: password,
            accounts: accounts,
        }
    }
}

impl Bank for Tangerine {
    fn kind(&self) -> &str {
        "tangerine"
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

impl Ofx for Tangerine {
    fn url(&self) -> &str {
        "https://ofx.tangerine.ca"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "10951"
    }

    fn fid_org(&self) -> &str {
        "TangerineBank"
    }

    fn bank_id(&self) -> &str {
        "00152614"
    }
}
