
use std::io::Read;
use reqwest;

pub(crate) fn HttpGet(url: &str) -> String{
    //发送http请求
    // let resp = reqwest::get(url)
    //     .json::<String>();
    //
    let mut resp = reqwest::blocking::get(url)?;
    let mut body = String::new();
    resp.read_to_string(&mut body)?;

    println!("{:#?}", body);
    body
}

#[test]
fn TestHttpGet(){
    HttpGet("https://sub.surfcloud.ink/api/v1/client/subscribe?token=4733b513f6122fa9477fdfc1500ff833");
}

//

//