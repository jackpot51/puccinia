use bank_ofx;

 bank_ofx!("fidelity", Fidelity, {
    fn url(&self) -> &str {
        "https://ofx.fidelity.com/ftgw/OFX/clients/download"
    }

    fn fid(&self) -> &str {
        "7776"
    }

    fn fid_org(&self) -> &str {
        "fidelity.com"
    }

    fn broker_id(&self) -> &str {
        "fidelity.com"
    }
});
