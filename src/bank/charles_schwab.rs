use bank_ofx;

bank_ofx!("charles_schwab", CharlesSchwab, {
    fn url(&self) -> &str {
        "https://ofx.schwab.com/cgi_dev/ofx_server"
    }

    fn fid(&self) -> &str {
        "1234"
    }

    fn fid_org(&self) -> &str {
        "SchwabRPS"
    }

    fn broker_id(&self) -> &str {
        "SchwabRPS.dv"
    }
});
