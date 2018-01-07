use chrono::{Date, TimeZone};
use ofx::Request;

pub fn ofx_request<'a, Tz: TimeZone>(user: &'a str, password: &'a str, account_id: &'a str, account_type: &'a str, start: &'a Date<Tz>, end: &'a Date<Tz>) -> Request<'a, Tz> {
    Request {
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
        account_id: account_id,
        account_type: account_type,
        start: start,
        end: end
    }
}
