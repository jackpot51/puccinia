use bank_ofx;

bank_ofx!("tangerine", Tangerine, {
    fn url(&self) -> &str {
        "https://ofx.tangerine.ca"
    }

    fn fid(&self) -> &str {
        "10951"
    }

    fn fid_org(&self) -> &str {
        "TangerineBank"
    }
});
