#![allow(unused)]

use std::collections::HashMap;

use raylib::drawing::RaylibDrawHandle;
use raylib::misc::AsF32;
use raylib::prelude::Rectangle;
use raylib::rgui::RaylibDrawGui;

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
          $($PName: crate::ui::UiValue::new(stringify!($PName).to_string()),)*
        }
      }
    }
  };
}

pub trait UiBuilder<T> {
  fn build(&mut self, draw: &mut impl RaylibDrawGui) -> T;
}

pub struct UiValue {
  id: String,
  element: Option<Box<dyn std::any::Any>>
}

impl UiValue {
  pub fn new(id: String) -> UiValue {
    UiValue { id, element: None }
  }

  pub fn slider<N: AsF32>(&mut self, rect: Rectangle, val: N, min: N, max: N) -> &mut SliderUiValue {
    if let None = self.element {
      let slider = SliderUiValue {
        rect,
        val: val.as_f32(),
        min: min.as_f32(),
        max: max.as_f32(),
      };

      self.element = Some(Box::new(slider));
    }

    if let Some(val) = &mut self.element {
      val.downcast_mut::<SliderUiValue>().unwrap()
    } else {
      panic!("How is this none?")
    }
  }

}

pub struct SliderUiValue {
  rect: Rectangle,
  val: f32,
  min: f32,
  max: f32,
}

impl<'a> UiBuilder<Option<f32>> for SliderUiValue {
  fn build(&mut self, draw: &mut impl RaylibDrawGui) -> Option<f32> {
    let val = draw.gui_slider(self.rect, None, None, self.val, self.min, self.max);

    if self.val != val {
      self.val = val;
      Some(self.val)
    } else {
      None
    }
  }
}
