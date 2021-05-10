pub mod theme;

#[allow(unused_macros)]
#[macro_export]
macro_rules! rayui_str {
  ($($arg:tt)*) => {Some(rstr!($($arg)*))};
}

#[cfg(test)]
mod tests {
  use raylib::prelude::*;

  use crate::theme::{Theme, RaylibHandleApplyTheme};
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

  #[test]
  fn test() {}

  #[test]
  fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, thread) = raylib::init()
      .size(1920, 1080)
      .title("Raylib UI Testing")
      .resizable()
      .msaa_4x()
      .build();

    #[rustfmt::skip]
    let theme = Theme::default()
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
      );

    rl.apply(&theme);

    let mut color = rcolor(32, 32, 32, 255);

    while !rl.window_should_close() {
      let mut d = rl.begin_drawing(&thread);

      d.clear_background(color);

      if d.gui_button(rrect(0, 0, 200, 100), rayui_str!("Yes.")) {}

      proto_state!(bool = true => d.gui_check_box(rrect(200, 0, 100, 100), None, _STATE));
      proto_state!(f32 = 0.0 => d.gui_slider(rrect(0, 100, 300, 100), None, None, _STATE, 0.0, 360.0));

      proto_state!(
        bool = true,
        _REF: [u8; 25] = *b"This is some random text\0"
        => d.gui_text_box(rrect(0, 200, 500, 300), &mut _REF, _STATE)
      );

      color = d.gui_color_picker(rrect(300, 0, 200, 200), color);

      // proto_state!(Color = Color::new(255, 0, 0, 255) => d.gui_color_picker(rrect(300, 0, 200, 200), _STATE) => {
      //
      // });
    }

    Ok(())
  }
}
