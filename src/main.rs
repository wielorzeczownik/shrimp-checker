#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod constants;
mod i18n;

use crate::app::{subscription, update, view, window_title, App};

fn main() -> iced::Result {
  iced::application(App::default, update, view)
    .title(window_title)
    .exit_on_close_request(false)
    .subscription(subscription)
    .run()
}
