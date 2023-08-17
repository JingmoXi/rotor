use reqwest;

pub(crate) async fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    //发送http请求
    // let resp = reqwest::get(url)
    //     .json::<String>();
    //
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;
    // use tokio::task::spawn_blocking;
    // let resp = spawn_blocking(move || {
    //     // let url = url;
    //     reqwest::blocking::get(url).unwrap().text().unwrap()
    // }).await;


    println!("{:#?}", resp);
    Ok(resp)
    //body
    //
}

#[test]
fn TestHttpGet() {
    http_get("https://sub.surfcloud.ink/api/v1/client/subscribe?token=4733b513f6122fa9477fdfc1500ff833");
    //
}

//
//