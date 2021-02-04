use bank_ofx;

bank_ofx!("vanguard", Vanguard, {
    fn url(&self) -> &str {
        "https://vesnc.vanguard.com/us/OfxDirectConnectServlet"
    }

    fn fid(&self) -> &str {
        "15103"
    }

    fn fid_org(&self) -> &str {
        "Vanguard"
    }

    fn broker_id(&self) -> &str {
        "vanguard.com"
    }
});
