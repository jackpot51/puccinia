use chrono::{Date, TimeZone};
use ofx::Request;

pub fn ofx_request<'a, Tz: TimeZone>(user: &'a str, password: &'a str, account_id: &'a str, start: &'a Date<Tz>, end: &'a Date<Tz>) -> Request<'a, Tz> {
    Request {
        url: "https://vesnc.vanguard.com/us/OfxDirectConnectServlet",
        ofx_ver: "103",

        user: user,
        password: password,
        language: "ENG",
        fid: "1358",
        fid_org: "Vanguard",
        app_id: "QBKS",
        app_ver: "1900",
        client_id: "",

        bank_id: "vanguard.com",
        account_id: account_id,
        account_type: "INVSTMT",
        start: start,
        end: end
    }
}
