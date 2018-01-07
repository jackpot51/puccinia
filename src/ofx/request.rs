use chrono::{Date, Local, TimeZone};
use rand::{Rng, thread_rng};
use std::fmt::Display;
use std::io::{self, Write};
use xml::writer::{EventWriter, EmitterConfig, XmlEvent, Error, Result};

pub fn random_string(len: usize) -> String {
    let mut string = String::with_capacity(len);

    let chars = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    let mut rng = thread_rng();
    for _ in 0..len {
        let i = rng.gen_range(0, chars.len());
        string.push(chars[i] as char);
    }

    string
}

pub fn date_string<Tz: TimeZone>(date: &Date<Tz>) -> String where Tz::Offset: Display {
    date.format("%Y%m%d").to_string()
}

//TODO
pub enum AccountType {
    Checking,
    Savings,
    Investment,
    MoneyMarket,
    CreditCard
}

pub struct Request<'a, Tz: TimeZone + 'a> {
    pub url: &'a str,
    pub ofx_ver: &'a str,

    pub user: &'a str,
    pub password: &'a str,
    pub language: &'a str,
    pub fid: &'a str,
    pub fid_org: &'a str,
    pub app_id: &'a str,
    pub app_ver: &'a str,
    pub client_id: &'a str,

    pub bank_id: &'a str,
    pub account_id: &'a str,
    pub account_type: &'a str,
    pub start: &'a Date<Tz>,
    pub end: &'a Date<Tz>,

    //
    //pub broker_id: &'a str,
}

impl<'a, Tz: TimeZone + 'a> Request<'a, Tz> {
    fn write_header<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "OFXHEADER:100\r\n")?;
        write!(w, "DATA:OFXSGML\r\n")?;
        write!(w, "VERSION:{}\r\n", self.ofx_ver)?;
        write!(w, "SECURITY:NONE\r\n")?;
        write!(w, "ENCODING:USASCII\r\n")?;
        write!(w, "CHARSET:1252\r\n")?;
        write!(w, "COMPRESSION:NONE\r\n")?;
        write!(w, "OLDFILEUID:NONE\r\n")?;
        write!(w, "NEWFILEUID:{}\r\n", random_string(32))?;
        write!(w, "\r\n")?;

        Ok(())
    }

    fn write_signon<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()> {
        w.write(XmlEvent::start_element("SIGNONMSGSRQV1"))?;
        {
            w.write(XmlEvent::start_element("SONRQ"))?;
            {
                w.write(XmlEvent::start_element("DTCLIENT"))?;
                w.write(XmlEvent::characters(&date_string(&Local::today())))?;

                w.write(XmlEvent::start_element("USERID"))?;
                w.write(XmlEvent::characters(self.user))?;

                w.write(XmlEvent::start_element("USERPASS"))?;
                w.write(XmlEvent::characters(self.password))?;

                w.write(XmlEvent::start_element("LANGUAGE"))?;
                w.write(XmlEvent::characters(self.language))?;

                w.write(XmlEvent::start_element("FI"))?;
                {
                    w.write(XmlEvent::start_element("ORG"))?;
                    w.write(XmlEvent::characters(self.fid_org))?;

                    w.write(XmlEvent::start_element("FID"))?;
                    w.write(XmlEvent::characters(self.fid))?;
                }
                w.write(XmlEvent::end_element().name("FI"))?;

                w.write(XmlEvent::start_element("APPID"))?;
                w.write(XmlEvent::characters(self.app_id))?;

                w.write(XmlEvent::start_element("APPVER"))?;
                w.write(XmlEvent::characters(self.app_ver))?;

                if ! self.client_id.is_empty() {
                    w.write(XmlEvent::start_element("CLIENTUID"))?;
                    w.write(XmlEvent::characters(self.client_id))?;
                }
            }
            w.write(XmlEvent::end_element().name("SONRQ"))?;
        }
        w.write(XmlEvent::end_element().name("SIGNONMSGSRQV1"))?;

        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>> where Tz::Offset: Display {
        let mut data = Vec::new();

        self.write_header(&mut data).map_err(|err| {
            Error::Io(err)
        })?;

        {
            let mut w = EmitterConfig::new()
                .perform_indent(false)
                .write_document_declaration(false)
                .normalize_empty_elements(false)
                .cdata_to_characters(false)
                .keep_element_names_stack(false)
                .autopad_comments(false)
                .create_writer(&mut data);

            match self.account_type {
                "INVESTMENT" => {
                },
                "CREDITCARD" => {
                },
                _ => {
                    w.write(XmlEvent::start_element("OFX"))?;
                    {
                        self.write_signon(&mut w)?;

                        w.write(XmlEvent::start_element("BANKMSGSRQV1"))?;
                        {
                            w.write(XmlEvent::start_element("STMTTRNRQ"))?;
                            {
                                w.write(XmlEvent::start_element("TRNUID"))?;
                                w.write(XmlEvent::characters(&random_string(32)))?;

                                w.write(XmlEvent::start_element("CLTCOOKIE"))?;
                                w.write(XmlEvent::characters(&random_string(5)))?;

                                w.write(XmlEvent::start_element("STMTRQ"))?;
                                {
                                    w.write(XmlEvent::start_element("BANKACCTFROM"))?;
                                    {
                                        w.write(XmlEvent::start_element("BANKID"))?;
                                        w.write(XmlEvent::characters(self.bank_id))?;

                                        w.write(XmlEvent::start_element("ACCTID"))?;
                                        w.write(XmlEvent::characters(self.account_id))?;

                                        w.write(XmlEvent::start_element("ACCTTYPE"))?;
                                        w.write(XmlEvent::characters(self.account_type))?;
                                    }
                                    w.write(XmlEvent::end_element().name("BANKACCTFROM"))?;

                                    w.write(XmlEvent::start_element("INCTRAN"))?;
                                    {
                                        w.write(XmlEvent::start_element("DTSTART"))?;
                                        w.write(XmlEvent::characters(&date_string(&self.start)))?;

                                        w.write(XmlEvent::start_element("DTEND"))?;
                                        w.write(XmlEvent::characters(&date_string(&self.end)))?;

                                        w.write(XmlEvent::start_element("INCLUDE"))?;
                                        w.write(XmlEvent::characters("Y"))?;
                                    }
                                    w.write(XmlEvent::end_element().name("INCTRAN"))?;
                                }
                                w.write(XmlEvent::end_element().name("STMTRQ"))?;
                            }
                            w.write(XmlEvent::end_element().name("STMTTRNRQ"))?;
                        }
                        w.write(XmlEvent::end_element().name("BANKMSGSRQV1"))?;
                    }
                    w.write(XmlEvent::end_element().name("OFX"))?;
                }
            }
        }

        Ok(data)
    }
}
