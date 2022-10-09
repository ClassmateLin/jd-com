/// Get account from env. eg: pt_pin=jd_1;pt_key=safasf;remark=test1;&pt_pin=jd_2;pt_key=saffsa;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Clone)]
pub struct JAccount {
    pt_pin: String,
    pt_key: String,
    remark: Option<String>,
}

impl JAccount {
    pub fn cookie(&self) -> String {
        format!("pt_pin={};pt_key={};", self.pt_pin, self.pt_key)
    }

    pub fn name(&self) -> String {
        match &self.remark {
            Some(remark) => remark.to_string(),
            None => self.pt_pin.to_string(),
        }
    }
}

pub fn get_accounts(jd_cookie: String) -> Vec<JAccount> {
    let mut array = Vec::new();
    let accounts: Vec<&str> = jd_cookie.split('&').filter(|s| !s.is_empty()).collect();
    let reg = Regex::new(r"(?P<key>[^?&=;]+)=(?P<val>[^=;]*)").unwrap();

    for account in accounts {
        let vals = reg.captures_iter(account);
        let mut data = json!({});
        for val in vals {
            data[val.name("key").unwrap().as_str()] =
                Value::from(val.name("val").unwrap().as_str());
        }
        array.push(data);
    }

    let res: Vec<JAccount> = serde_json::from_value(json!(array)).unwrap();

    res
}

#[cfg(test)]
mod tests {
    use super::get_accounts;

    #[test]
    fn test_get_accounts() {
        let cookie = "pt_pin=jd_1;pt_key=safasf;remark=test1;&pt_pin=jd_2;pt_key=saffsa;";
        let accounts = get_accounts(cookie.to_string());
        assert_eq!(2, accounts.len());
        assert_eq!("test1".to_string(), accounts[0].name());
        assert_eq!(
            "pt_pin=jd_1;pt_key=safasf;".to_string(),
            accounts[0].cookie()
        );
        assert_eq!(
            "pt_pin=jd_2;pt_key=saffsa;".to_string(),
            accounts[1].cookie()
        );
        assert_eq!("jd_2".to_string(), accounts[1].name());
    }
}
