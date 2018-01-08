use decimal::d128;

use bank::{Bank, BankAccount};
use ofx::Ofx;

pub struct Amex {
    username: String,
    password: String,
    accounts: Option<Vec<BankAccount>>,
}

impl Amex {
    pub fn new(username: String, password: String, accounts: Option<Vec<BankAccount>>) -> Amex {
        Amex {
            username: username,
            password: password,
            accounts: accounts,
        }
    }
}

impl Bank for Amex {
    fn name(&self) -> &str {
        "amex"
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

impl Ofx for Amex {
    fn url(&self) -> &str {
        "https://online.americanexpress.com/myca/ofxdl/desktop/desktopDownload.do?request_type=nl_ofxdownload"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "3101"
    }

    fn fid_org(&self) -> &str {
        "AMEX"
    }

    fn app_id(&self) -> &str {
        "QWIN"
    }

    fn app_ver(&self) -> &str {
        "1500"
    }
}
