
use iced::widget::{button, Column, Text};
use iced::{ Element, Length, Sandbox, Settings};

struct  Vpn{
    name: String,//名称
    address: String,//地址
    delay: String,//延迟
    used: bool,//使用中
    status: String,//是否可用
}

#[derive(Debug, Clone, Copy)]
pub enum Message{
    SwitchOn,//开启
    SwitchOff,//关闭
    ChooseLine,//选择路线
    //
}

impl Sandbox for Vpn{
    type Message = Message;

    fn new() -> Self {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SwitchOn => {
                //todo,开启
            }
            Message::SwitchOff => {
                //todo,关闭
            }
            Message::ChooseLine => {
                //todo,选择路线
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        todo!()
    }
}