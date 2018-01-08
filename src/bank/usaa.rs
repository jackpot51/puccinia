use decimal::d128;

use bank::{Bank, BankAccount};
use ofx::Ofx;

pub struct Usaa {
    username: String,
    password: String,
    accounts: Option<Vec<BankAccount>>,
}

impl Usaa {
    pub fn new(username: String, password: String, accounts: Option<Vec<BankAccount>>) -> Usaa {
        Usaa {
            username: username,
            password: password,
            accounts: accounts,
        }
    }
}

impl Bank for Usaa {
    fn name(&self) -> &str {
        "usaa"
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
}

impl Ofx for Usaa {
    fn url(&self) -> &str {
        "https://service2.usaa.com/ofx/OFXServlet"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "24591"
    }

    fn fid_org(&self) -> &str {
        "USAA"
    }

    fn bank_id(&self) -> &str {
        "314074269"
    }

    fn broker_id(&self) -> &str {
        "USAA.COM"
    }
}
