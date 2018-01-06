use std::io::{self, Write};
use xml::writer::{EventWriter, EmitterConfig, XmlEvent, Error, Result};

pub struct Request<'a> {
    pub url: &'a str,
    pub ofx_ver: &'a str,

    pub time: &'a str,
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
    pub start: &'a str,
    pub end: &'a str,

    //
    //pub broker_id: &'a str,
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
        write!(w, "NEWFILEUID:{}\r\n", "TODO")?;

        Ok(())
    }

    fn write_signon<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()> {
        w.write(XmlEvent::start_element("SIGNONMSGSRQV1"))?;
        {
            w.write(XmlEvent::start_element("SONRQ"))?;
            {
                w.write(XmlEvent::start_element("DTCLIENT"))?;
                w.write(XmlEvent::characters(self.time))?;
                w.write(XmlEvent::end_element())?;

                w.write(XmlEvent::start_element("USERID"))?;
                w.write(XmlEvent::characters(self.user))?;
                w.write(XmlEvent::end_element())?;

                w.write(XmlEvent::start_element("USERPASS"))?;
                w.write(XmlEvent::characters(self.password))?;
                w.write(XmlEvent::end_element())?;

                w.write(XmlEvent::start_element("LANGUAGE"))?;
                w.write(XmlEvent::characters(self.language))?;
                w.write(XmlEvent::end_element())?;

                w.write(XmlEvent::start_element("FI"))?;
                {
                    w.write(XmlEvent::start_element("ORG"))?;
                    w.write(XmlEvent::characters(self.fid_org))?;
                    w.write(XmlEvent::end_element())?;

                    w.write(XmlEvent::start_element("FID"))?;
                    w.write(XmlEvent::characters(self.fid))?;
                    w.write(XmlEvent::end_element())?;
                }
                w.write(XmlEvent::end_element())?;

                w.write(XmlEvent::start_element("APPID"))?;
                w.write(XmlEvent::characters(self.app_id))?;
                w.write(XmlEvent::end_element())?;

                w.write(XmlEvent::start_element("APPVER"))?;
                w.write(XmlEvent::characters(self.app_ver))?;
                w.write(XmlEvent::end_element())?;

                w.write(XmlEvent::start_element("CLIENTUID"))?;
                w.write(XmlEvent::characters(self.client_id))?;
                w.write(XmlEvent::end_element())?;
            }
            w.write(XmlEvent::end_element())?;
        }
        w.write(XmlEvent::end_element())?;

        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        self.write_header(&mut data).map_err(|err| {
            Error::Io(err)
        })?;

        {
            let mut w = EmitterConfig::new()
                .perform_indent(true)
                .write_document_declaration(false)
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
                                w.write(XmlEvent::characters("TODO"))?;
                                w.write(XmlEvent::end_element())?;

                                w.write(XmlEvent::start_element("CLTCOOKIE"))?;
                                w.write(XmlEvent::characters("TODO"))?;
                                w.write(XmlEvent::end_element())?;

                                w.write(XmlEvent::start_element("STMTRQ"))?;
                                {
                                    w.write(XmlEvent::start_element("BANKACCTFROM"))?;
                                    {
                                        w.write(XmlEvent::start_element("BANKID"))?;
                                        w.write(XmlEvent::characters(self.bank_id))?;
                                        w.write(XmlEvent::end_element())?;

                                        w.write(XmlEvent::start_element("ACCTID"))?;
                                        w.write(XmlEvent::characters(self.account_id))?;
                                        w.write(XmlEvent::end_element())?;

                                        w.write(XmlEvent::start_element("ACCTTYPE"))?;
                                        w.write(XmlEvent::characters(self.account_type))?;
                                        w.write(XmlEvent::end_element())?;
                                    }
                                    w.write(XmlEvent::end_element())?;

                                    w.write(XmlEvent::start_element("INCTRAN"))?;
                                    {
                                        w.write(XmlEvent::start_element("DTSTART"))?;
                                        w.write(XmlEvent::characters(self.start))?;
                                        w.write(XmlEvent::end_element())?;

                                        w.write(XmlEvent::start_element("DTEND"))?;
                                        w.write(XmlEvent::characters(self.end))?;
                                        w.write(XmlEvent::end_element())?;

                                        w.write(XmlEvent::start_element("INCLUDE"))?;
                                        w.write(XmlEvent::characters("Y"))?;
                                        w.write(XmlEvent::end_element())?;
                                    }
                                    w.write(XmlEvent::end_element())?;
                                }
                                w.write(XmlEvent::end_element())?;
                            }
                            w.write(XmlEvent::end_element())?;
                        }
                        w.write(XmlEvent::end_element())?;
                    }
                    w.write(XmlEvent::end_element())?;
                }
            }
        }

        Ok(data)
    }
}
