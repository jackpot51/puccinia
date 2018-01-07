use ofx::Ofx;

pub struct Amex {
    username: String,
    password: String,
}

impl Amex {
    pub fn new(username: String, password: String) -> Amex {
        Amex {
            username: username,
            password: password,
        }
    }
}

impl Ofx for Amex {
    fn url(&self) -> &str {
        "https://online.americanexpress.com/myca/ofxdl/desktop/desktopDownload.do?request_type=nl_ofxdownload"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "3101"
    }

    fn fid_org(&self) -> &str {
        "AMEX"
    }

    fn app_id(&self) -> &str {
        "QWIN"
    }

    fn app_ver(&self) -> &str {
        "1500"
    }
}
