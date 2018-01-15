use std::collections::BTreeMap;
use std::str;
use xml::reader::{ParserConfig, XmlEvent, Result};

fn header_length(data: &[u8]) -> Option<usize> {
    for search in &["\r\n\r\n", "\n\n"] {
        for len in search.len()..data.len() {
            if &data[len - search.len() .. len] == search.as_bytes() {
                return Some(len);
            }
        }
    }

    None
}

#[derive(Debug, Default)]
pub struct Account {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub bank_id: Option<String>,
    pub broker_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct Balance {
    pub amount: Option<String>,
    pub time: Option<String>,
}

#[derive(Debug, Default)]
pub struct Position {
    pub id: Option<String>,
    pub id_kind: Option<String>,
    pub time: Option<String>,
    pub held_in_account: Option<String>,
    pub memo: Option<String>,
    pub market_value: Option<String>,
    pub kind: Option<String>,
    pub unit_price: Option<String>,
    pub units: Option<String>,
}

#[derive(Debug, Default)]
pub struct Security {
    pub id: Option<String>,
    pub id_kind: Option<String>,
    pub name: Option<String>,
    pub ticker: Option<String>,
    pub memo: Option<String>,
    pub unit_price: Option<String>,
}

#[derive(Debug, Default)]
pub struct Transaction {
    pub id: Option<String>,
    pub time: Option<String>,
    pub name: Option<String>,
    pub memo: Option<String>,
    pub amount: Option<String>,
    pub kind: Option<String>,
    pub check_num: Option<String>,
    pub ref_num: Option<String>,
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
    pub positions: Vec<Position>,
    pub securities: Vec<Security>,
    pub transactions: Vec<Transaction>,
}

impl Response {
    pub fn decode(data: &[u8]) -> Result<Response> {
        let mut response = Response::default();

        let header_len = match header_length(data) {
            Some(len) => len,
            None => {
                panic!("Header not terminated. Produce an error!\n{:?}", str::from_utf8(data));
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

        let mut security_id = None;
        let mut security_id_kind = None;
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

                                "OFX/CREDITCARDMSGSRSV1/CCSTMTTRNRS/CCSTMTRS" |
                                "OFX/BANKMSGSRSV1/STMTTRNRS/STMTRS" |
                                "OFX/INVSTMTMSGSRSV1/INVSTMTTRNRS/INVSTMTRS" => {
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
                                        broker_id: stack_data.remove("BROKERID")
                                    });
                                },

                                "OFX/SIGNUPMSGSRSV1/ACCTINFOTRNRS/ACCTINFORS/ACCTINFO/CCACCTINFO/CCACCTFROM" |
                                "OFX/CREDITCARDMSGSRSV1/CCSTMTTRNRS/CCSTMTRS/CCACCTFROM" => {
                                    println!("Account");
                                    response.accounts.push(Account {
                                        id: stack_data.remove("ACCTID"),
                                        kind: Some("CREDITCARD".to_string()),
                                        bank_id: None,
                                        broker_id: None
                                    });
                                },

                                "OFX/INVSTMTMSGSRSV1/INVSTMTTRNRS/INVSTMTRS/INVACCTFROM" => {
                                    println!("Account");
                                    response.accounts.push(Account {
                                        id: stack_data.remove("ACCTID"),
                                        kind: Some("INVESTMENT".to_string()),
                                        bank_id: None,
                                        broker_id: stack_data.remove("BROKERID")
                                    });
                                },

                                "OFX/CREDITCARDMSGSRSV1/CCSTMTTRNRS/CCSTMTRS/LEDGERBAL" |
                                "OFX/BANKMSGSRSV1/STMTTRNRS/STMTRS/LEDGERBAL" => {
                                    println!("Balance");
                                    response.balance = Some(Balance {
                                        amount: stack_data.remove("BALAMT"),
                                        time: stack_data.remove("DTASOF"),
                                    });
                                },

                                "OFX/CREDITCARDMSGSRSV1/CCSTMTTRNRS/CCSTMTRS/BANKTRANLIST" |
                                "OFX/BANKMSGSRSV1/STMTTRNRS/STMTRS/BANKTRANLIST" |
                                "OFX/INVSTMTMSGSRSV1/INVSTMTTRNRS/INVSTMTRS/INVTRANLIST" => {
                                    println!("Transaction list");
                                    response.start = stack_data.remove("DTSTART");
                                    response.end = stack_data.remove("DTEND");
                                },

                                "OFX/CREDITCARDMSGSRSV1/CCSTMTTRNRS/CCSTMTRS/BANKTRANLIST/STMTTRN" |
                                "OFX/BANKMSGSRSV1/STMTTRNRS/STMTRS/BANKTRANLIST/STMTTRN" |
                                "OFX/INVSTMTMSGSRSV1/INVSTMTTRNRS/INVSTMTRS/INVTRANLIST/INVBANKTRAN/STMTTRN" => {
                                    println!("Transaction");
                                    response.transactions.push(Transaction {
                                        id: stack_data.remove("FITID"),
                                        time: stack_data.remove("DTPOSTED"),
                                        name: stack_data.remove("NAME"),
                                        memo: stack_data.remove("MEMO"),
                                        amount: stack_data.remove("TRNAMT"),
                                        kind: stack_data.remove("TRNTYPE"),
                                        check_num: stack_data.remove("CHECKNUM"),
                                        ref_num: stack_data.remove("REFNUM"),
                                    });
                                },

                                "OFX/INVSTMTMSGSRSV1/INVSTMTTRNRS/INVSTMTRS/INVPOSLIST/POSMF/INVPOS" => {
                                    println!("Position");
                                    response.positions.push(Position {
                                        id: security_id.take(),
                                        id_kind: security_id_kind.take(),
                                        time: stack_data.remove("DTPRICEASOF"),
                                        held_in_account: stack_data.remove("HELDINACCT"),
                                        memo: stack_data.remove("MEMO"),
                                        market_value: stack_data.remove("MKTVAL"),
                                        kind: stack_data.remove("POSTYPE"),
                                        unit_price: stack_data.remove("UNITPRICE"),
                                        units: stack_data.remove("UNITS")
                                    });
                                },

                                "OFX/INVSTMTMSGSRSV1/INVSTMTTRNRS/INVSTMTRS/INVPOSLIST/POSMF/INVPOS/SECID" => {
                                    println!("Position security id");
                                    security_id = stack_data.remove("UNIQUEID");
                                    security_id_kind = stack_data.remove("UNIQUEIDTYPE");
                                },

                                "OFX/SECLISTMSGSRSV1/SECLIST/MFINFO/SECINFO" => {
                                    println!("Security info");
                                    response.securities.push(Security {
                                        id: security_id.take(),
                                        id_kind: security_id_kind.take(),
                                        name: stack_data.remove("SECNAME"),
                                        ticker: stack_data.remove("TICKER"),
                                        memo: stack_data.remove("MEMO"),
                                        unit_price: stack_data.remove("UNITPRICE")
                                    });
                                },

                                "OFX/SECLISTMSGSRSV1/SECLIST/MFINFO/SECINFO/SECID" => {
                                    println!("Security info id");
                                    security_id = stack_data.remove("UNIQUEID");
                                    security_id_kind = stack_data.remove("UNIQUEIDTYPE");
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
