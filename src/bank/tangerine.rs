use ofx::Ofx;

pub struct Tangerine {
    username: String,
    password: String,
}

impl Tangerine {
    pub fn new(username: String, password: String) -> Tangerine {
        Tangerine {
            username: username,
            password: password,
        }
    }
}

impl Ofx for Tangerine {
    fn url(&self) -> &str {
        "https://ofx.tangerine.ca"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "10951"
    }

    fn fid_org(&self) -> &str {
        "TangerineBank"
    }

    fn bank_id(&self) -> &str {
        "00152614"
    }
}
