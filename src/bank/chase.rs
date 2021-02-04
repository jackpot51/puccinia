use bank_ofx;

bank_ofx!("chase", Chase, {
    fn url(&self) -> &str {
        "https://ofx.chase.com"
    }

    fn ofx_ver(&self) -> &str {
        "103"
    }

    fn fid(&self) -> &str {
        "10898"
    }

    fn fid_org(&self) -> &str {
        "B1"
    }

    fn app_id(&self) -> &str {
        "QWIN"
    }

    fn app_ver(&self) -> &str {
        "2500"
    }
});
