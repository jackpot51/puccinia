use ofx::bank::Bank;

pub struct Tangerine<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> Tangerine<'a> {
    pub fn new(username: &'a str, password: &'a str) -> Tangerine<'a> {
        Tangerine {
            username: username,
            password: password,
        }
    }
}

impl<'a> Bank for Tangerine<'a> {
    fn url(&self) -> &str {
        "https://ofx.tangerine.ca"
    }

    fn username(&self) -> &str {
        self.username
    }

    fn password(&self) -> &str {
        self.password
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
