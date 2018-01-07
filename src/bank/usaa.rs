use ofx::Ofx;

pub struct Usaa {
    username: String,
    password: String,
}

impl Usaa {
    pub fn new(username: String, password: String) -> Usaa {
        Usaa {
            username: username,
            password: password,
        }
    }
}

impl Ofx for Usaa {
    fn url(&self) -> &str {
        "https://service2.usaa.com/ofx/OFXServlet"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
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
