use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, button, column, container, image, row, text};
use iced::window;
use iced::{ContentFit, Element, Length, Subscription, Task};

use crate::constants::{
  BUTTON_SPACING, CAT_BYTES, CONTENT_PADDING, CONTENT_SPACING, SHRIMP_BYTES, SHRIMP_SIZE,
  SOUND_BYTES, TITLE_SIZE,
};
use crate::i18n::{Text, load_strings};

/// Messages produced by UI actions and window events.
#[derive(Debug, Clone)]
pub(crate) enum Message {
  Yes,
  No,
  RequestClose,
  GotOldestWindowId(Option<window::Id>),
  WindowCloseRequested(window::Id),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
  Question,
  Cat,
}

/// Application state.
pub struct App {
  screen: Screen,
  cat: image::Handle,
  shrimp: image::Handle,
  is_playing: bool,
  strings: Text,
}

impl Default for App {
  fn default() -> Self {
    let cat = image::Handle::from_bytes(CAT_BYTES.to_vec());
    let shrimp = image::Handle::from_bytes(SHRIMP_BYTES.to_vec());
    let strings = load_strings();

    Self {
      screen: Screen::Question,
      cat,
      shrimp,
      is_playing: false,
      strings,
    }
  }
}

/// Provides the localized window title.
pub(crate) fn window_title(app: &App) -> String {
  app.strings.window_title.to_string()
}

/// Subscribes to window close requests so we can block them during playback.
pub(crate) fn subscription(_app: &App) -> Subscription<Message> {
  window::close_requests().map(Message::WindowCloseRequested)
}

/// Updates application state and schedules side-effects.
pub(crate) fn update(app: &mut App, message: Message) -> Task<Message> {
  match message {
    Message::Yes => {
      app.screen = Screen::Cat;
      app.is_playing = true;
      play_yes_music_task()
    }
    Message::No => Task::done(Message::RequestClose),
    Message::RequestClose => {
      app.is_playing = false;
      window::oldest().map(Message::GotOldestWindowId)
    }

    Message::GotOldestWindowId(Some(id)) => window::close(id),
    Message::GotOldestWindowId(None) => Task::none(),
    Message::WindowCloseRequested(id) => {
      if app.is_playing {
        Task::none()
      } else {
        window::close(id)
      }
    }
  }
}

/// Renders the current screen.
pub(crate) fn view(app: &App) -> Element<'_, Message> {
  match app.screen {
    Screen::Question => view_question(&app.strings, &app.shrimp),
    Screen::Cat => view_cat(&app.cat),
  }
}

fn view_question<'a>(strings: &'a Text, shrimp: &'a image::Handle) -> Element<'a, Message> {
  let shrimp_img = image(shrimp.clone())
    .width(Length::Fixed(SHRIMP_SIZE))
    .height(Length::Fixed(SHRIMP_SIZE))
    .content_fit(ContentFit::Contain);

  let title = text(strings.question)
    .size(TITLE_SIZE)
    .align_x(Horizontal::Center);

  let buttons = row![
    button(text(strings.yes)).on_press(Message::Yes),
    button(text(strings.no)).on_press(Message::No),
  ]
  .spacing(BUTTON_SPACING)
  .align_y(Vertical::Center);

  let content: Column<Message> = column![shrimp_img, title, buttons]
    .spacing(CONTENT_SPACING)
    .align_x(Horizontal::Center);

  container(content)
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .padding(CONTENT_PADDING)
    .into()
}

fn view_cat(cat: &image::Handle) -> Element<'static, Message> {
  let img = image(cat.clone())
    .width(Length::Fill)
    .height(Length::Fill)
    .content_fit(ContentFit::Cover);

  container(img)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn play_yes_music_task() -> Task<Message> {
  Task::perform(
    async {
      play_yes_music_blocking();
    },
    |_| Message::RequestClose,
  )
}

/// Plays the confirmation sound on the audio thread.
fn play_yes_music_blocking() {
  let Ok(mut sink_handle) = rodio::DeviceSinkBuilder::open_default_sink() else {
    return;
  };
  sink_handle.log_on_drop(false);
  let player = rodio::Player::connect_new(sink_handle.mixer());

  let cursor = std::io::Cursor::new(SOUND_BYTES);
  let Ok(source) = rodio::Decoder::new(cursor) else {
    return;
  };

  player.append(source);
  player.sleep_until_end();
}
