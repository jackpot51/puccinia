use chrono::{Date, Utc};

pub use self::amex::Amex;
pub use self::usaa::Usaa;
pub use self::vanguard::Vanguard;

mod amex;
mod usaa;
mod vanguard;

use ofx::{ofx, Request, Response};

pub trait Bank {
    fn url(&self) -> &str;

    fn ofx_ver(&self) -> &str {
        "102"
    }

    fn username(&self) -> &str;

    fn password(&self) -> &str;

    fn language(&self) -> &str {
        "ENG"
    }

    fn fid(&self) -> &str;

    fn fid_org(&self) -> &str;

    fn app_id(&self) -> &str {
        "QBKS"
    }

    fn app_ver(&self) -> &str {
        "1900"
    }

    fn client_id(&self) -> &str {
        ""
    }

    fn bank_id(&self) -> &str {
        ""
    }

    fn broker_id(&self) -> &str {
        ""
    }

    fn ofx(&self, account_id: &str, account_type: &str, start: Option<Date<Utc>>, end: Option<Date<Utc>>) -> Result<Response, String> {
        ofx(Request {
            url: self.url(),
            ofx_ver: self.ofx_ver(),

            username: self.username(),
            password: self.password(),
            language: self.language(),
            fid: self.fid(),
            fid_org: self.fid_org(),
            app_id: self.app_id(),
            app_ver: self.app_ver(),
            client_id: self.client_id(),

            bank_id: self.bank_id(),
            broker_id: self.broker_id(),
            account_id: account_id,
            account_type: account_type,

            start: start,
            end: end
        })
    }

    fn accounts(&self) -> Result<Response, String> {
        self.ofx("", "", None, None)
    }
}
