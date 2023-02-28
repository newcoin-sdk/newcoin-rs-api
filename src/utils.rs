use crate::io_sys::io::NCInitUrlsDev;
use crate::eos_api::api_types::AnyType;
use reqwest::{Error, Response, Client, Request};
use std::collections::HashMap;
struct NcoReadApi { // We must split this struct out.
    aa_url: String,
    hyp_url: String,
}


struct OptStruct;
struct DataOpts;
struct AssetApiParams;
struct IAsset;
type Opts<K,V> = HashMap<K, V>;

fn build_data_options(options: OptStruct, data: DataOpts) {
    unimplemented!();
}

fn fetch_endpoint<'a, T>(url: String, page: T, limit: T, data: DataOpts) -> DataOpts{
    DataOpts
}
async fn get_assets(options: OptStruct, data: DataOpts, page: i32, limit: i32) -> Result<Vec<IAsset>, Error> {
    let data_opts = build_data_options(options, data);
    fetch_endpoint("/v1/assets".into(), page, limit, DataOpts);

    unimplemented!();
}

impl NcoReadApi {
    fn new() -> NcoReadApi {
        let nc_init = NCInitUrlsDev::default();
        NcoReadApi {
            aa_url: nc_init.atomicassets_url,
            hyp_url: nc_init.hyperion_url,
        }
    }

    /// Makes a call to the hyperion_url.
    
    pub async fn read_tx(self, tx_id: String) -> Result<Response, Error> {
        
        let init_client = Client::builder().build().unwrap();

        let get_req = init_client
            .get(self.hyp_url.to_string() + &"/v2/history/get_transaction?id=tx_id".to_string())
            .send().await;

        get_req
        
        
    }

    pub async fn read_asset(asset_id: String) {

    }
}