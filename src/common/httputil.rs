use std::collections::HashMap;
use reqwest;

pub(crate) fn HttpGet(url: &str) ->Result<String>{
    //发送http请求
    let resp = reqwest::get(url)
        .json::<HashMap<String, String>>();
    //
    return ""
}

#[test]
fn TestHttpGet(){
    HttpGet("https://sub.surfcloud.ink/api/v1/client/subscribe?token=4733b513f6122fa9477fdfc1500ff833");
}

//

//