mod en;
mod pl;

#[derive(Debug, Clone, Copy)]
pub struct Text {
  pub window_title: &'static str,
  pub question: &'static str,
  pub yes: &'static str,
  pub no: &'static str,
}

pub use en::TEXT_EN;
pub use pl::TEXT_PL;

/// Loads localized UI text based on the system locale.
pub fn load_strings() -> Text {
  let locale = sys_locale::get_locale().unwrap_or_default().to_lowercase();
  if locale.starts_with("pl") {
    TEXT_PL
  } else {
    TEXT_EN
  }
}
