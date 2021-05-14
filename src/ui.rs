#![allow(unused)]

use std::collections::HashMap;

use raylib::drawing::RaylibDrawHandle;
use raylib::misc::AsF32;
use raylib::prelude::Rectangle;
use raylib::rgui::RaylibDrawGui;
use std::ffi::CString;

#[macro_export]
macro_rules! ids {
  ($Vis:vis struct $Name:ident {
    $($PName:ident),* $(,)?
  }) => {
    $Vis struct $Name {
      $($PName: crate::ui::UiElementState,)*
    }
    impl $Name {
      fn new() -> Self {
        Self {
          $($PName: crate::ui::UiElementState::new(stringify!($PName).to_string()),)*
        }
      }
    }
  };
}

pub trait UiBuilder<T> {
  fn build(&mut self, draw: &mut impl RaylibDrawGui) -> T;
}

pub struct UiElementState {
  id: String,
  element: Option<Box<dyn std::any::Any>>,
}

impl UiElementState {
  pub fn new(id: String) -> UiElementState {
    UiElementState { id, element: None }
  }

  fn mutable_ref<T: std::any::Any>(&mut self) -> &mut T {
    if let Some(val) = &mut self.element {
      val.downcast_mut::<T>().unwrap()
    } else {
      panic!("How is this none?")
    }
  }

  pub fn button<S: Into<String>>(&mut self, rect: Rectangle, text: S) -> &mut Button {
    if let None = self.element {
      let mut button = Button {
        rect,
        text: Some(CString::new(text.into()).unwrap())
      };

      self.element = Some(Box::new(button));
    }

    self.mutable_ref()
  }

  pub fn slider<N: AsF32>(&mut self, rect: Rectangle, val: N, min: N, max: N) -> &mut Slider {
    if let None = self.element {
      let slider = Slider {
        rect,
        val: val.as_f32(),
        min: min.as_f32(),
        max: max.as_f32(),
      };

      self.element = Some(Box::new(slider));
    }

    self.mutable_ref()
  }
}

pub struct Button {
  rect: Rectangle,
  text: Option<CString>,
}

impl Button {
  pub fn set_text(&mut self, text: String) {
    self.text = Some(CString::new(text).unwrap());
  }
}

impl UiBuilder<bool> for Button {
  fn build(&mut self, draw: &mut impl RaylibDrawGui) -> bool {
    draw.gui_button(self.rect, self.text.as_ref().map(|it| it.as_c_str()))
  }
}

pub struct Slider {
  rect: Rectangle,
  val: f32,
  min: f32,
  max: f32,
}

impl UiBuilder<Option<f32>> for Slider {
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
