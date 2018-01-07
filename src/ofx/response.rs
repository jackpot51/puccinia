use std::collections::BTreeMap;
use xml::reader::{ParserConfig, XmlEvent, Result};

fn header_length(data: &[u8]) -> Option<usize> {
    let search = b"\r\n\r\n";

    for len in search.len()..data.len() {
        if &data[len - search.len() .. len] == search {
            return Some(len);
        }
    }

    None
}

#[derive(Debug, Default)]
pub struct Account {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub bank_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct Balance {
    pub amount: Option<String>,
    pub time: Option<String>,
}

#[derive(Debug, Default)]
pub struct Transaction {
    pub check_num: Option<String>,
    pub time: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub amount: Option<String>,
    pub kind: Option<String>,
}

#[derive(Debug, Default)]
pub struct Response {
    pub fid: Option<String>,
    pub fid_org: Option<String>,
    pub time: Option<String>,
    pub language: Option<String>,
    pub currency: Option<String>,
    pub accounts: Vec<Account>,
    pub balance: Option<Balance>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub transactions: Vec<Transaction>,
}

impl Response {
    pub fn decode(data: &[u8]) -> Result<Response> {
        let mut response = Response::default();

        let header_len = match header_length(data) {
            Some(len) => len,
            None => {
                panic!("Header not terminated. Produce an error!");
            }
        };

        let mut xml_data = &data[header_len ..];

        let r = ParserConfig::new()
            .trim_whitespace(true)
            .whitespace_to_characters(false)
            .cdata_to_characters(false)
            .ignore_comments(true)
            .coalesce_characters(false)
            .ignore_end_of_stream(false)
            .create_reader(&mut xml_data);

        let mut stack = Vec::new();
        for e_res in r {
            let e = e_res?;
            //println!("{:?}", e);

            match e {
                XmlEvent::StartElement { name, .. } => {
                    stack.push((name.local_name, BTreeMap::new()));
                },
                XmlEvent::Characters(value) => {
                    if let Some((stack_name, _)) = stack.pop() {
                        if let Some(parent) = stack.last_mut() {
                            parent.1.insert(stack_name, value);
                        } else {
                            panic!("Parent not found in stack");
                        }
                    } else {
                        panic!("Characters outside of stack");
                    }
                },
                XmlEvent::EndElement { name, .. } => {
                    if let Some((stack_name, mut stack_data)) = stack.pop() {
                        if name.local_name == stack_name {
                            let mut stack_path = String::new();
                            for entry in &stack {
                                stack_path.push_str(&entry.0);
                                stack_path.push('/');
                            }
                            stack_path.push_str(&stack_name);

                            print!("{}: ", stack_path);

                            match stack_path.as_str() {
                                "OFX/SIGNONMSGSRSV1/SONRS" => {
                                    println!("SONRS");
                                    response.time = stack_data.remove("DTSERVER");
                                    response.language = stack_data.remove("LANGUAGE");
                                },

                                "OFX/SIGNONMSGSRSV1/SONRS/FI" => {
                                    println!("Financial Institution");
                                    response.fid = stack_data.remove("FID");
                                    response.fid_org = stack_data.remove("ORG");
                                },

                                "OFX/BANKMSGSRSV1/STMTTRNRS/STMTRS" => {
                                    println!("Statement");
                                    response.currency = stack_data.remove("CURDEF");
                                },

                                "OFX/SIGNUPMSGSRSV1/ACCTINFOTRNRS/ACCTINFORS/ACCTINFO/BANKACCTINFO/BANKACCTFROM" |
                                "OFX/BANKMSGSRSV1/STMTTRNRS/STMTRS/BANKACCTFROM" => {
                                    println!("Account");
                                    response.accounts.push(Account {
                                        id: stack_data.remove("ACCTID"),
                                        kind: stack_data.remove("ACCTTYPE"),
                                        bank_id: stack_data.remove("BANKID"),
                                    });
                                },

                                "OFX/BANKMSGSRSV1/STMTTRNRS/STMTRS/LEDGERBAL" => {
                                    println!("Balance");
                                    response.balance = Some(Balance {
                                        amount: stack_data.remove("BALAMT"),
                                        time: stack_data.remove("DTASOF"),
                                    });
                                },

                                "OFX/BANKMSGSRSV1/STMTTRNRS/STMTRS/BANKTRANLIST" => {
                                    println!("Transaction list");
                                    response.start = stack_data.remove("DTSTART");
                                    response.end = stack_data.remove("DTEND");
                                },

                                "OFX/BANKMSGSRSV1/STMTTRNRS/STMTRS/BANKTRANLIST/STMTTRN" => {
                                    println!("Transaction");
                                    response.transactions.push(Transaction {
                                        check_num: stack_data.remove("CHECKNUM"),
                                        time: stack_data.remove("DTPOSTED"),
                                        id: stack_data.remove("FITID"),
                                        name: stack_data.remove("NAME"),
                                        amount: stack_data.remove("TRNAMT"),
                                        kind: stack_data.remove("TRNTYPE"),
                                    });
                                },

                                _ => {
                                    println!("Unknown");
                                }
                            }

                            for (key, value) in &stack_data {
                                println!("    Unused: {}={}", key, value);
                            }
                        } else {
                            panic!("EndElement name {} did not match stack name {}", name.local_name, stack_name);
                        }
                    } else {
                        panic!("EndElement outside of stack");
                    }
                }
                _ => ()
            }
        }

        Ok(response)
    }
}
