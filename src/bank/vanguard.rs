use ofx::Ofx;

pub struct Vanguard {
    username: String,
    password: String,
}

impl Vanguard {
    pub fn new(username: String, password: String) -> Vanguard {
        Vanguard {
            username: username,
            password: password,
        }
    }
}

impl Ofx for Vanguard {
    fn url(&self) -> &str {
        "https://vesnc.vanguard.com/us/OfxDirectConnectServlet"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "1358"
    }

    fn fid_org(&self) -> &str {
        "Vanguard"
    }

    fn broker_id(&self) -> &str {
        "vanguard.com"
    }
}
