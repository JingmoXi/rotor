
use std::fmt;
use serde::Serialize;

// use log::{info};
#[allow(dead_code)]
#[derive(Serialize)]
pub(crate) struct ShadowSockts {
    pub(crate) name: String,
    pub(crate) addr: String,
    pub(crate) port: i32,
    pub(crate) ciper: String,
    pub(crate) password: String,
}

impl fmt::Display for ShadowSockts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}:{}:{}", self.name, self.addr, self.port, self.ciper,self.password)
    }
}

// impl fmt::Display for Vec<ShadowSockts> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}",self.len())
//     }
// }



#[allow(dead_code)]
fn connect(){
    // #[allow(dead_code)]
    // let proxies: Vec<ShadowSockts> = vec![];
    // //打印日志信息
    // info!("Connecting");
    //
    //
}

#[allow(dead_code)]
//todo: 解析订阅地址
fn subscribe(){
    //解析

}




//