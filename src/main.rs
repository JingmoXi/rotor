#![deny(warnings)]


mod common;
mod openapi;
mod proxy;
mod dashboard;


// use std::error::Error;
use bytes::Buf;
use futures_util::{stream, StreamExt};
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
use common::httputil;
use base64;
use proxy::model;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INDEX: &[u8] = b"<a href=\"test.html\">test.html</a>";
static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";
static POST_DATA: &str = r#"{"original": "data"}"#;
static URL: &str = "http://127.0.0.1:1337/json_api";

async fn client_request_response(client: &Client<HttpConnector>) -> Result<Response<Body>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri(URL)
        .header(header::CONTENT_TYPE, "application/json")
        .body(POST_DATA.into())
        .unwrap();

    let web_res = client.request(req).await?;
    // Compare the JSON we sent (before) with what we received (after):
    let before = stream::once(async {
        Ok(format!(
            "<b>POST request body</b>: {}<br><b>Response</b>: ",
            POST_DATA,
        )
            .into())
    });
    let after = web_res.into_body();
    let body = Body::wrap_stream(before.chain(after));

    Ok(Response::new(body))
}





async unsafe fn subsribes() -> Result<Response<Body>> {


    let res = match serde_json::to_string(&PROXYY) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };
    Ok(res)
}

async unsafe fn route(
    req: Request<Body>,
    client: Client<HttpConnector>,
) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => Ok(Response::new(INDEX.into())),
        (&Method::GET, "/test.html") => client_request_response(&client).await,
        (&Method::GET, "/subsribes") => subsribes().await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

static  mut PROXYY: Vec<model::ShadowSockts> = vec![];

#[tokio::main]
async fn main() -> Result<()> {
    //pretty_env_logger::init();
    //
    //
    let url = "https://sub.surfcloud.ink/api/v1/client/subscribe?token=4733b513f6122fa9477fdfc1500ff833";

    let subscribe_info = match parse_subscribeurl(url).await {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            panic!("subscribe url error!")
        }
    };
    unsafe {   get_proxy_infos(subscribe_info); }
    //println!("{}", subscribe_info);

    //-----------------------------启动ui界面------------------------------------------------
    //todo,


    let addr = "127.0.0.1:9000".parse().unwrap();
    //lib::establish_connection();
    // Share a `Client` with all `Service`s
    let client = Client::new();

    let new_service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        let client = client.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| unsafe {
                // Clone again to ensure that client outlives this closure.
                route(req, client.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);



    server.await?;

    Ok(())
}

#[allow(dead_code)]
unsafe fn get_proxy_infos(links: String)  {
    //
    let linkvec:Vec<&str> = links.split_terminator('\n').collect();
    //
    for link in linkvec {
        //
        println!("{}",link);
        PROXYY.push(model::ShadowSockts{
            name: "".to_string(),
            addr: link.to_string(),
            port: 8080,
            ciper: "".to_string(),
            password: "".to_string(),
        });
        println!("{}",PROXYY.len());
    };
    //借用？
    // println!{"{}",Proxy};


}

async fn parse_subscribeurl(url: &str) -> Result<String> {
    //获取订阅信息
    let resp = httputil::http_get(url);
    let re = match resp.await {
        Ok(text) => text,
        Err(err) => return Err(GenericError::try_from(err.to_string()).unwrap()),
    };

    use base64::{Engine as _, engine::{general_purpose}};
    let b64 = general_purpose::STANDARD.decode(re);


    let k = match b64 {
        Ok(text) => String::from_utf8(text),
        Err(err) => return Err(GenericError::try_from(err.to_string()).unwrap()),
    };

    match k {
        Ok(text) => return Ok(text),
        Err(err) => return Err(GenericError::try_from(err.to_string()).unwrap()),
    };


    // let decoded_bytes = Engine::encode_string(resp)?;
    // let decoded_string = String::from_utf8(decoded_bytes)?;
}


//