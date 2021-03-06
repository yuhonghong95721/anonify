use std::collections::HashMap;
use reqwest::{Client, header};
use anyhow::{Result, anyhow};
use anonify_runtime::{Bytes, UpdatedState, traits::State};
use serde_json::Value;

const ENDPOINT_TRANSFER_REQUEST: &str = "https://api.sunabar.gmo-aozora.com/personal/v1/transfer/request";
const TRANSFER_DATA: &str = r#"{
  "accountId": "301010000338",
  "remitterName": "ｵｽｹ",
  "transferDesignatedDate": "2020-06-18",
  "transferDateHolidayCode": "1",
  "totalCount": "1",
  "totalAmount": "0",
  "transfers": [
    {
      "transferAmount": "0",
      "beneficiaryBankCode": "0398",
      "beneficiaryBankName": "ｱｵｿﾞﾗ",
      "beneficiaryBranchCode": "111",
      "beneficiaryBranchName": "ﾎｳｼﾞﾝｴｲｷﾞｮｳﾌﾞ",
      "accountTypeCode": "1",
      "accountNumber": "0000314",
      "beneficiaryName": "ｽﾅﾊﾞｺｳｽｹ(ｶ"
    }
  ]
}"#;

lazy_static! {
    static ref SUNABAR_SECRET: String = {
        use std::env;
        let secret = env::var("SUNABAR_SECRET").unwrap();
        format!("{}{}", "Bearer ", secret)
    };
}

#[derive(Debug, Clone)]
pub struct SunabarClient {
    client: Client,
    body: Value,
}

impl SunabarClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", header::HeaderValue::from_static("application/json;charset=UTF-8"));
        headers.insert("x-access-token", header::HeaderValue::from_static(&SUNABAR_SECRET));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("SunabarClient builder failed");
        let body: Value = serde_json::from_str(TRANSFER_DATA).unwrap();

        SunabarClient {
            client,
            body,
        }
    }

    pub fn set_shared_invoice(mut self, invoice: &UpdatedState<Bytes>) -> Self {
        let invoice_json: Value = serde_json::from_slice(&invoice.state.as_bytes()).unwrap();
        let amount = &invoice_json["data"][0]["attributes"]["total_price"];

        *self.body.get_mut("totalAmount").unwrap() = amount.clone();
        *self.body.get_mut("transfers").unwrap()
            .get_mut(0).unwrap()
            .get_mut("transferAmount").unwrap() = amount.clone();

        self
    }

    pub fn transfer_request(self) -> Result<String> {
        self.client
            .post(ENDPOINT_TRANSFER_REQUEST)
            .json(&self.body)
            .send()?
            .text()
            .map_err(Into::into)
    }
}
