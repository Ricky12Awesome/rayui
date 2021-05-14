pub mod theme;
pub mod ui;

#[allow(unused_macros)]
#[macro_export]
macro_rules! rayui_str {
  ($($arg:tt)*) => {Some(rstr!($($arg)*))};
}

#[cfg(test)]
mod tests {
  use raylib::prelude::*;

  use crate::theme::{RaylibHandleApplyTheme, Theme};
  use crate::ui::{UiBuilder};
  use crate::*;

  #[allow(unused_macros)]
  macro_rules! proto_state {
    ($T:tt = $D:expr $(, $N:ident: $TT:ty = $DD:expr)* => $E:expr $(=> $B:block)?) => {
      unsafe {
        static mut _STATE: $T = $D;
        $(static mut $N: $TT = $DD;)*
        let _new_state = $E;
        $(if _new_state != _STATE $B;)?
        _STATE = _new_state; _STATE
      }
    };
  }

  ids! {
    struct Ids {
      slider,
    }
  }

  #[test]
  fn test() {}

  #[rustfmt::skip]
  fn theme() -> Theme {
    Theme::default()
      .control_theme(|it| it
        .border_width(2)
      )
      .slider_theme(|it| it
        .slider_padding(4)
        .slider_width(4)
      )
      .checkbox_theme(|it| it
        .check_padding(8)
        .control_theme(|it| it
          // for some reason combobox uses text color instead of base
          .text_normal(Color::RED)
        )
      )
  }

  #[test]
  fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, thread) = raylib::init()
      .size(1920, 1080)
      .title("Raylib UI Testing")
      .resizable()
      .msaa_4x()
      .build();

    let theme = theme();

    rl.apply(&theme);

    let color = rcolor(32, 32, 32, 255);

    let mut ids = Ids::new();

    while !rl.window_should_close() {
      let mut d = rl.begin_drawing(&thread);

      d.clear_background(Color::new(32, 32, 32, 32));

      if let Some(val) = ids.slider
        .slider(rrect(5.0, 5.0, 400.0, 40.0), 0, -10, 10)
        .build(&mut d)
      {
        println!("{}", val);
      }
    }

    Ok(())
  }
}
