use ofx::Ofx;

pub struct Usaa<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> Usaa<'a> {
    pub fn new(username: &'a str, password: &'a str) -> Usaa<'a> {
        Usaa {
            username: username,
            password: password,
        }
    }
}

impl<'a> Ofx for Usaa<'a> {
    fn url(&self) -> &str {
        "https://service2.usaa.com/ofx/OFXServlet"
    }

    fn username(&self) -> &str {
        self.username
    }

    fn password(&self) -> &str {
        self.password
    }

    fn fid(&self) -> &str {
        "24591"
    }

    fn fid_org(&self) -> &str {
        "USAA"
    }

    fn bank_id(&self) -> &str {
        "314074269"
    }

    fn broker_id(&self) -> &str {
        "USAA.COM"
    }
}
