use bank_ofx;

bank_ofx!("usaa", Usaa, {
    fn url(&self) -> &str {
        "https://df3cx-services.1fsapi.com/casm/usaa/access.ofx"
    }

    fn ofx_ver(&self) -> &str {
        "103"
    }

    fn pretty(&self) -> bool {
        true
    }

    fn fid(&self) -> &str {
        "67811"
    }

    fn fid_org(&self) -> &str {
        "USAA Federal Savings Bank"
    }

    fn app_id(&self) -> &str {
        "QMOFX"
    }

    fn app_ver(&self) -> &str {
        "2300"
    }

    fn broker_id(&self) -> &str {
        "USAA.COM"
    }
});
