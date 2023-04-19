//! # rs_ncdao
//! rs_ncdao is an api-wrapper, for the newcoin api/protocol, predicated on EOS.IO
//! rs_ncdao uses reqwest/tokio to manage api calls and responses.
//! As of yet there are very few dependencies, mainly serde for de/serializing
//! objects so that they can be distributed beyond the Rust SDK,
//! i.e for SDK's in other languages, or web-apps.
//! This functionality will also be made available via compilation to web assembly.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#[macro_use]
extern crate dotenv_codegen;
mod c_api;
mod eos_api;
mod json_rpc;
mod ncdao;
mod types;
mod utils;

use reqwest::{Error, Response, Client, Request};

use crate::c_api::chain_api::{get_table_rows, ChainApi, get_table_rows_with_payload, get_proposal_by_id};
use crate::eos_api::api_types::AnyType;
use io_sys::io::NCInit;
use types::{GetTableRowsPayload, ProposalPayload};

#[tokio::main]
async fn main() -> Result<(), Error> {

        
    let proposal_payload = ProposalPayload {
        id: "1".to_string(),
        contract: "daos2.nco".to_string(),
    };

    let payload = GetTableRowsPayload {
        json: true,
        code: proposal_payload.contract.clone(),
        scope: proposal_payload.contract.clone(),
        table: "slice".into(),
        table_key: proposal_payload.id.clone(),
        lower_bound: proposal_payload.id.clone(),
        upper_bound: proposal_payload.id,
        key_type: "i64".into(),
        index_position: "1".into(),
        encode_type: "".into(),
        limit: 0,
        reverse: false,
        show_payer: false,
    };
    let gtr_response = get_table_rows().await;
    let data_1 = gtr_response?.text().await?;

    let gtr_wp_response = get_proposal_by_id(&payload).await;

    let data_2 = gtr_wp_response.unwrap_err();
    let gtr_wp: Response = get_table_rows_with_payload(&payload).await.unwrap();

    println!("Content 1: {:?} \n", data_1);
    println!("Content 2: {:?}", data_2);

    Ok(())

}
