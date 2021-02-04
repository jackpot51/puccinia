use bank_ofx;

bank_ofx!("fidelity_nb", FidelityNb, {
    fn url(&self) -> &str {
        "https://nbofx.fidelity.com/netbenefits/ofx/download"
    }

    fn fid(&self) -> &str {
        "8288"
    }

    fn fid_org(&self) -> &str {
        "nbofx.fidelity.com"
    }

    fn broker_id(&self) -> &str {
        "nbofx.fidelity.com"
    }
});
