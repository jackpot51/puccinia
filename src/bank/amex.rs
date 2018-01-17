use bank::{Bank, BankAccount};
use ofx::Ofx;

pub struct Amex {
    name: String,
    username: String,
    password: String,
    accounts: Option<Vec<BankAccount>>,
}

impl Amex {
    pub fn new(name: String, username: String, password: String, accounts: Option<Vec<BankAccount>>) -> Amex {
        Amex {
            name: name,
            username: username,
            password: password,
            accounts: accounts,
        }
    }
}

impl Bank for Amex {
    fn kind(&self) -> &str {
        "amex"
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
