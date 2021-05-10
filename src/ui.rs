#![allow(unused)]

use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::Rectangle;
use raylib::misc::AsF32;
use raylib::rgui::RaylibDrawGui;
use std::collections::HashMap;

#[macro_export]
macro_rules! ids {
    ($Vis:vis struct $Name:ident {
      $($PName:ident),* $(,)?
    }) => {
      $Vis struct $Name {
        $($PName: crate::ui::UiValue,)*
      }

      impl $Name {
        fn new() -> Self {
          Self {
            $($PName: crate::ui::UiValue::default(),)*
          }
        }
      }
    };
}

#[derive(Default)]
pub struct UiValue {
  bytes: [u8; 4]
}

impl UiValue {
  fn set_u32(&mut self, val: u32) {
    self.bytes = val.to_be_bytes();
  }

  fn get_u32(&self) -> u32 {
    u32::from_be_bytes(self.bytes)
  }

  fn set_f32(&mut self, val: f32) {
    self.bytes = val.to_be_bytes();
  }

  fn get_f32(&self) -> f32 {
    f32::from_be_bytes(self.bytes)
  }

  fn set_bool(&mut self, val: bool) {
    self.bytes = if val { [1, 0, 0, 0] } else { [0, 0, 0, 0] };
  }

  fn get_bool(&self) -> bool {
    self.bytes[0] != 0
  }
}

pub trait Ui: RaylibDrawGui {
  #[rustfmt::skip]
   fn slider<N: AsF32>(
    &mut self, rect:
    impl Into<Rectangle>,
    min: N,
    max: N,
    val: &mut UiValue
  ) -> Option<f32> {
    let old_val = val.get_f32();

    let new_val = self.gui_slider(
      rect,
      None,
      None,
      old_val,
      min.as_f32(),
      max.as_f32()
    );

    if old_val != new_val {
      val.set_f32(new_val);

      Some(new_val)
    } else {
      None
    }
  }
}

impl <T: RaylibDrawGui> Ui for T {

}
