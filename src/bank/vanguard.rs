use ofx::Ofx;

pub struct Vanguard<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> Vanguard<'a> {
    pub fn new(username: &'a str, password: &'a str) -> Vanguard<'a> {
        Vanguard {
            username: username,
            password: password,
        }
    }
}

impl<'a> Ofx for Vanguard<'a> {
    fn url(&self) -> &str {
        "https://vesnc.vanguard.com/us/OfxDirectConnectServlet"
    }

    fn username(&self) -> &str {
        self.username
    }

    fn password(&self) -> &str {
        self.password
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
