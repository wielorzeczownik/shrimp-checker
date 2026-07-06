#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod i18n;

use crate::app::{App, subscription, update, view, window_title};

fn main() -> iced::Result {
  iced::application(App::default, update, view)
    .title(window_title)
    .exit_on_close_request(false)
    .subscription(subscription)
    .run()
}
