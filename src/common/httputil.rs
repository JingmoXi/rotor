

use reqwest;

pub(crate) fn http_get(url: &str)->Result<String, Box<dyn std::error::Error>>  {
    //发送http请求
    // let resp = reqwest::get(url)
    //     .json::<String>();
    //
    let  resp = reqwest::blocking::get(url)?.text()?;


    println!("{:#?}", resp);
    Ok(resp)
    //body
    //
}

#[test]
fn TestHttpGet(){
    http_get("https://sub.surfcloud.ink/api/v1/client/subscribe?token=4733b513f6122fa9477fdfc1500ff833");
    //
}

//
//