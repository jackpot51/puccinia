use chrono::{Date, Utc};
use rand::{Rng, thread_rng};
use std::io::{self, Write};
use xml::writer::{EventWriter, EmitterConfig, XmlEvent, Error, Result};

use super::date_to_string;

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

pub struct Request<'a> {
    pub url: &'a str,
    pub ofx_ver: &'a str,
    pub pretty: bool,

    pub username: &'a str,
    pub password: &'a str,
    pub language: &'a str,
    pub fid: &'a str,
    pub fid_org: &'a str,
    pub app_id: &'a str,
    pub app_ver: &'a str,
    pub client_id: &'a str,

    pub broker_id: &'a str,
    pub bank_id: &'a str,
    pub account_id: &'a str,
    pub account_type: &'a str,
    pub start: Option<Date<Utc>>,
    pub end: Option<Date<Utc>>,
}

impl<'a> Request<'a> {
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
        if ! self.pretty {
            // When using pretty formatting, first tag will output endline
            write!(w, "\r\n")?;
        }

        Ok(())
    }

    fn start_element<W: Write>(&self, w: &mut EventWriter<W>, element: &str) -> Result<()> {
        if self.pretty {
            write!(w.inner_mut(), "\r\n")?;
        }
        w.write(XmlEvent::start_element(element))
    }

    fn end_element<W: Write>(&self, w: &mut EventWriter<W>, element: &str) -> Result<()> {
        if self.pretty {
            write!(w.inner_mut(), "\r\n")?;
        }
        w.write(XmlEvent::end_element().name(element))
    }

    fn write_signon<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()> {
        self.start_element(w, "SIGNONMSGSRQV1")?;
        {
            self.start_element(w, "SONRQ")?;
            {
                self.start_element(w, "DTCLIENT")?;
                w.write(XmlEvent::characters(&date_to_string(&Utc::today())))?;

                self.start_element(w, "USERID")?;
                w.write(XmlEvent::characters(self.username))?;

                self.start_element(w, "USERPASS")?;
                w.write(XmlEvent::characters(self.password))?;

                self.start_element(w, "LANGUAGE")?;
                w.write(XmlEvent::characters(self.language))?;

                self.start_element(w, "FI")?;
                {
                    self.start_element(w, "ORG")?;
                    w.write(XmlEvent::characters(self.fid_org))?;

                    self.start_element(w, "FID")?;
                    w.write(XmlEvent::characters(self.fid))?;
                }
                self.end_element(w, "FI")?;

                self.start_element(w, "APPID")?;
                w.write(XmlEvent::characters(self.app_id))?;

                self.start_element(w, "APPVER")?;
                w.write(XmlEvent::characters(self.app_ver))?;

                if ! self.client_id.is_empty() {
                    self.start_element(w, "CLIENTUID")?;
                    w.write(XmlEvent::characters(self.client_id))?;
                }
            }
            self.end_element(w, "SONRQ")?;
        }
        self.end_element(w, "SIGNONMSGSRQV1")?;

        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        self.write_header(&mut data).map_err(|err| {
            Error::Io(err)
        })?;

        {
            let mut writer = EmitterConfig::new()
                .perform_indent(false)
                .write_document_declaration(false)
                .normalize_empty_elements(false)
                .cdata_to_characters(false)
                .keep_element_names_stack(false)
                .autopad_comments(false)
                .create_writer(&mut data);
            let w = &mut writer;

            match self.account_type {
                "" => {
                    self.start_element(w, "OFX")?;
                    {
                        self.write_signon(w)?;

                        self.start_element(w, "SIGNUPMSGSRQV1")?;
                        {
                            self.start_element(w, "ACCTINFOTRNRQ")?;
                            {
                                self.start_element(w, "TRNUID")?;
                                w.write(XmlEvent::characters(&random_string(32)))?;

                                self.start_element(w, "ACCTINFORQ")?;
                                {
                                    self.start_element(w, "DTACCTUP")?;
                                    //TODO: make configurable
                                    w.write(XmlEvent::characters("19900101"))?;
                                }
                                self.end_element(w, "ACCTINFORQ")?;
                            }
                            self.end_element(w, "ACCTINFOTRNRQ")?;
                        }
                        self.end_element(w, "SIGNUPMSGSRQV1")?;
                    }
                    self.end_element(w, "OFX")?;
                },
                "INVESTMENT" => {
                    self.start_element(w, "OFX")?;
                    {
                        self.write_signon(w)?;

                        self.start_element(w, "INVSTMTMSGSRQV1")?;
                        {
                            self.start_element(w, "INVSTMTTRNRQ")?;
                            {
                                self.start_element(w, "TRNUID")?;
                                w.write(XmlEvent::characters(&random_string(32)))?;

                                self.start_element(w, "CLTCOOKIE")?;
                                w.write(XmlEvent::characters(&random_string(5)))?;

                                self.start_element(w, "INVSTMTRQ")?;
                                {
                                    self.start_element(w, "INVACCTFROM")?;
                                    {
                                        self.start_element(w, "BROKERID")?;
                                        w.write(XmlEvent::characters(self.broker_id))?;

                                        self.start_element(w, "ACCTID")?;
                                        w.write(XmlEvent::characters(self.account_id))?;
                                    }
                                    self.end_element(w, "INVACCTFROM")?;

                                    self.start_element(w, "INCTRAN")?;
                                    {
                                        if let Some(ref start) = self.start {
                                            self.start_element(w, "DTSTART")?;
                                            w.write(XmlEvent::characters(&date_to_string(start)))?;
                                        }

                                        if let Some(ref end) = self.end {
                                            self.start_element(w, "DTEND")?;
                                            w.write(XmlEvent::characters(&date_to_string(end)))?;
                                        }

                                        self.start_element(w, "INCLUDE")?;
                                        w.write(XmlEvent::characters("Y"))?;
                                    }
                                    self.end_element(w, "INCTRAN")?;

                                    self.start_element(w, "INCOO")?;
                                    w.write(XmlEvent::characters("Y"))?;

                                    self.start_element(w, "INCPOS")?;
                                    {
                                        self.start_element(w, "INCLUDE")?;
                                        w.write(XmlEvent::characters("Y"))?;
                                    }
                                    self.end_element(w, "INCPOS")?;

                                    self.start_element(w, "INCBAL")?;
                                    w.write(XmlEvent::characters("Y"))?;
                                }
                                self.end_element(w, "INVSTMTRQ")?;
                            }
                            self.end_element(w, "INVSTMTTRNRQ")?;
                        }
                        self.end_element(w, "INVSTMTMSGSRQV1")?;
                    }
                    self.end_element(w, "OFX")?;
                },
                "CREDITCARD" => {
                    self.start_element(w, "OFX")?;
                    {
                        self.write_signon(w)?;

                        self.start_element(w, "CREDITCARDMSGSRQV1")?;
                        {
                            self.start_element(w, "CCSTMTTRNRQ")?;
                            {
                                self.start_element(w, "TRNUID")?;
                                w.write(XmlEvent::characters(&random_string(32)))?;

                                self.start_element(w, "CLTCOOKIE")?;
                                w.write(XmlEvent::characters(&random_string(5)))?;

                                self.start_element(w, "CCSTMTRQ")?;
                                {
                                    self.start_element(w, "CCACCTFROM")?;
                                    {
                                        self.start_element(w, "ACCTID")?;
                                        w.write(XmlEvent::characters(self.account_id))?;
                                    }
                                    self.end_element(w, "CCACCTFROM")?;

                                    self.start_element(w, "INCTRAN")?;
                                    {
                                        if let Some(ref start) = self.start {
                                            self.start_element(w, "DTSTART")?;
                                            w.write(XmlEvent::characters(&date_to_string(start)))?;
                                        }

                                        if let Some(ref end) = self.end {
                                            self.start_element(w, "DTEND")?;
                                            w.write(XmlEvent::characters(&date_to_string(end)))?;
                                        }

                                        self.start_element(w, "INCLUDE")?;
                                        w.write(XmlEvent::characters("Y"))?;
                                    }
                                    self.end_element(w, "INCTRAN")?;
                                }
                                self.end_element(w, "CCSTMTRQ")?;
                            }
                            self.end_element(w, "CCSTMTTRNRQ")?;
                        }
                        self.end_element(w, "CREDITCARDMSGSRQV1")?;
                    }
                    self.end_element(w, "OFX")?;
                },
                _ => {
                    self.start_element(w, "OFX")?;
                    {
                        self.write_signon(w)?;

                        self.start_element(w, "BANKMSGSRQV1")?;
                        {
                            self.start_element(w, "STMTTRNRQ")?;
                            {
                                self.start_element(w, "TRNUID")?;
                                w.write(XmlEvent::characters(&random_string(32)))?;

                                self.start_element(w, "CLTCOOKIE")?;
                                w.write(XmlEvent::characters(&random_string(5)))?;

                                self.start_element(w, "STMTRQ")?;
                                {
                                    self.start_element(w, "BANKACCTFROM")?;
                                    {
                                        self.start_element(w, "BANKID")?;
                                        w.write(XmlEvent::characters(self.bank_id))?;

                                        self.start_element(w, "ACCTID")?;
                                        w.write(XmlEvent::characters(self.account_id))?;

                                        self.start_element(w, "ACCTTYPE")?;
                                        w.write(XmlEvent::characters(self.account_type))?;
                                    }
                                    self.end_element(w, "BANKACCTFROM")?;

                                    self.start_element(w, "INCTRAN")?;
                                    {
                                        if let Some(ref start) = self.start {
                                            self.start_element(w, "DTSTART")?;
                                            w.write(XmlEvent::characters(&date_to_string(start)))?;
                                        }

                                        if let Some(ref end) = self.end {
                                            self.start_element(w, "DTEND")?;
                                            w.write(XmlEvent::characters(&date_to_string(end)))?;
                                        }

                                        self.start_element(w, "INCLUDE")?;
                                        w.write(XmlEvent::characters("Y"))?;
                                    }
                                    self.end_element(w, "INCTRAN")?;
                                }
                                self.end_element(w, "STMTRQ")?;
                            }
                            self.end_element(w, "STMTTRNRQ")?;
                        }
                        self.end_element(w, "BANKMSGSRQV1")?;
                    }
                    self.end_element(w, "OFX")?;
                }
            }
        }

        Ok(data)
    }
}
