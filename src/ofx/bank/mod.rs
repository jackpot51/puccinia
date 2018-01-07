use chrono::{Date, Local, TimeZone};
use std::fmt::Display;

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

    fn ofx<Tz: TimeZone>(&self, account_id: &str, account_type: &str, start: &Date<Tz>, end: &Date<Tz>) -> Result<Response, String> where Tz::Offset: Display {
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
        self.ofx("", "", &Local::today(), &Local::today())
    }


    /*
    pub fn usaa<'a, Tz: TimeZone>(
        user: &'a str, password: &'a str,
        account_id: &'a str, account_type: &'a str,
        start: &'a Date<Tz>, end: &'a Date<Tz>
    ) -> Result<Response, String> where Tz::Offset: Display {
        ofx(Request {
            url: "https://service2.usaa.com/ofx/OFXServlet",
            ofx_ver: "102",

            user: user,
            password: password,
            language: "ENG",
            fid: "24591",
            fid_org: "USAA",
            app_id: "QBKS",
            app_ver: "1900",
            client_id: "",

            bank_id: "314074269",
            broker_id: "USAA.COM",
            account_id: account_id,
            account_type: account_type,
            start: start,
            end: end
        })
    }
    */
}
