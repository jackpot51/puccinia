use bank_ofx;

bank_ofx!("usaa", Usaa, {
    fn url(&self) -> &str {
        "https://service2.usaa.com/ofx/OFXServlet"
    }

    fn fid(&self) -> &str {
        "24591"
    }

    fn fid_org(&self) -> &str {
        "USAA"
    }

    fn broker_id(&self) -> &str {
        "USAA.COM"
    }
});
