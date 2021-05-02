#![allow(dead_code)]

use raylib::prelude::*;

macro_rules! impl_theme_value {
  ($($T:ty),+) => {
    $(impl ThemeValue for $T {
      fn theme_value(&self) -> i32 { *self as i32 }
    })+
  };
  ($($T:ty => $F:ident),+) => {
    $(impl ThemeValue for $T {
      fn theme_value(&self) -> i32 { self.$F() }
    })+
  };
}

macro_rules! all_optional {
  ($(#[$Attr:ident($($AttrValue:tt),*)])* $Vis:vis struct $Name:ident {
    $($PropVis:vis $Prop:ident: $Type:ty),* $(,)?
  }) => {
    $(#[$Attr($($AttrValue),*)])*
    $Vis struct $Name {
      $($PropVis $Prop: Option<$Type>),*
    }
  };
}

macro_rules! impl_default_for_optionals {
  ($Struct:ident, $($Prop:ident: $Type:ty $(=> $Default:expr)?),* $(,)?) => {
    impl Default for $Struct {
      fn default() -> Self {
        Self {
          $($Prop: {let _val = Option::<$Type>::None; $(let _val = Some($Default);)? _val}),*
        }
      }
    }
  };
}

macro_rules! impl_theme_struct {
  ($($Prop:ident: $Type:ident),* $(,)?) => {
    all_optional! {
      #[derive(Debug, Clone)]
      pub struct Theme {
        $(pub $Prop: $Type),*
      }
    }

    impl Theme {
      $(pub fn $Prop<F: FnOnce($Type) -> $Type>(mut self, f: F) -> Self {
        self.$Prop = Some(f(self.$Prop.clone().unwrap_or_default())); self
      })*
    }

    impl_default_for_optionals!(Theme, $($Prop: $Type => $Type::default()),*);

    impl Theme {
      pub fn properties(&self) -> Vec<(GuiControl, Vec<(i32, i32)>)> {
        let mut map = Vec::new();

        $(if let Some(theme) = &self.$Prop {
          map.push((theme.control(), theme.properties()));
        })*

        map
      }
    }
  };
}

macro_rules! impl_theme {
  ($SName:ident(no_control_theme): $Control:expr => {
    $($PName:ident: $PType:ty = $PProp:expr $(=> $Default:expr)?),* $(,)?
  }) => {
    all_optional! {
      #[derive(Debug, Clone)]
      pub struct $SName {
        $(pub $PName: $PType),*
      }
    }

    impl $SName {
      $(pub fn $PName(mut self, value: $PType) -> Self {
        self.$PName = Some(value); self
      })*
    }

    impl_default_for_optionals!($SName, $($PName: $PType $(=> $Default)?),*);

    impl ThemeProperties for $SName {
      fn control(&self) -> GuiControl {
        $Control
      }

      fn properties(&self) -> Vec<(i32, i32)> {
        let mut props = vec![];

        $(if let Some(value) = &self.$PName {
          props.push(($PProp as i32, value.theme_value()));
        })*

        props
      }
    }
  };
  ($SName:ident: $Control:expr => {
    $($PName:ident: $PType:ty = $PProp:expr $(=> $Default:expr)?),* $(,)?
  }) => {
    all_optional! {
      #[derive(Debug, Clone)]
      pub struct $SName {
        pub control_theme: ControlTheme,
        $(pub $PName: $PType,)*
      }
    }

    impl $SName {
      pub fn control_theme<F: FnOnce(ControlTheme) -> ControlTheme>(mut self, f: F) -> Self {
        self.control_theme = Some(f(self.control_theme.clone().unwrap_or_default())); self
      }

      $(pub fn $PName(mut self, value: $PType) -> Self {
        self.$PName = Some(value); self
      })*
    }

    impl_default_for_optionals!($SName, control_theme: ControlTheme, $($PName: $PType $(=> $Default)?),*);

    impl ThemeProperties for $SName {
      fn control(&self) -> GuiControl {
        $Control
      }

      fn properties(&self) -> Vec<(i32, i32)> {
        let mut props = vec![];

        if let Some(base) = &self.control_theme {
          props.extend(base.properties());
        }

        $(if let Some(value) = &self.$PName {
          props.push(($PProp as i32, value.theme_value()));
        })*

        props
      }
    }
  };
}

fn apply_theme<T, F: Fn(&mut T, GuiControl, i32, i32)>(t: &mut T, gui_set_style: F, theme: &Theme) {
  for (control, props) in theme.properties() {
    for (prop, value) in props {
      gui_set_style(t, control, prop, value);
    }
  }
}

pub trait RaylibDrawGuiApplyTheme {
  fn apply(&mut self, theme: &Theme);
}

pub trait RaylibHandleApplyTheme {
  fn apply(&mut self, theme: &Theme);
}

impl<T: RaylibDrawGui> RaylibDrawGuiApplyTheme for T {
  fn apply(&mut self, theme: &Theme) {
    apply_theme(self, Self::gui_set_style, theme);
  }
}

impl RaylibHandleApplyTheme for RaylibHandle {
  fn apply(&mut self, theme: &Theme) {
    apply_theme(self, Self::gui_set_style, theme);
  }
}

pub trait ThemeProperties {
  fn control(&self) -> GuiControl;
  fn properties(&self) -> Vec<(i32, i32)>;
}

pub trait ThemeValue {
  fn theme_value(&self) -> i32;
}

impl_theme_value!(Color => color_to_int);
impl_theme_value!(GuiTextAlignment, GuiScrollBarSide, GuiControlState, bool, u32, i32);

impl_theme_struct! {
  control_theme: ControlTheme,
  default_theme: DefaultTheme,
  toggle_theme: ToggleTheme,
  slider_theme: SliderTheme,
  progressbar_theme: ProgressBarTheme,
  checkbox_theme: CheckBoxTheme,
  combobox_theme: ComboBoxTheme,
  dropdownbox_theme: DropdownBoxTheme,
  textbox_theme: TextBoxTheme,
  scrollbar_theme: ScrollBarTheme,
  listview_theme: ListViewTheme,
  colorpicker_theme: ColorPickerTheme
}

impl_theme! {
 ToggleTheme: GuiControl::TOGGLE => {
   group_padding: u32 = GuiToggleProperty::GROUP_PADDING,
 }
}

impl_theme! {
  SliderTheme: GuiControl::SLIDER => {
    slider_width: u32 = GuiSliderProperty::SLIDER_WIDTH,
    slider_padding: u32 = GuiSliderProperty::SLIDER_PADDING,
  }
}

impl_theme! {
 ProgressBarTheme: GuiControl::PROGRESSBAR => {
   progress_padding: u32 = GuiProgressBarProperty::PROGRESS_PADDING,
 }
}

impl_theme! {
 CheckBoxTheme: GuiControl::CHECKBOX => {
   check_padding: u32 = GuiCheckBoxProperty::CHECK_PADDING,
 }
}

impl_theme! {
 ComboBoxTheme: GuiControl::COMBOBOX => {
   combo_button_width: u32 = GuiComboBoxProperty::COMBO_BUTTON_WIDTH,
   combo_button_padding: u32 = GuiComboBoxProperty::COMBO_BUTTON_PADDING,
 }
}

impl_theme! {
 DropdownBoxTheme: GuiControl::DROPDOWNBOX => {
   arrow_padding: u32 = GuiDropdownBoxProperty::ARROW_PADDING,
   dropdown_item_padding: u32 = GuiDropdownBoxProperty::DROPDOWN_ITEMS_PADDING,
 }
}

impl_theme! {
 TextBoxTheme: GuiControl::TEXTBOX => {
   text_inner_padding: u32 = GuiTextBoxProperty::TEXT_INNER_PADDING,
   text_lines_padding: u32 = GuiTextBoxProperty::TEXT_LINES_PADDING,
   color_selected_fg: Color = GuiTextBoxProperty::COLOR_SELECTED_FG,
   color_selected_bg: Color = GuiTextBoxProperty::COLOR_SELECTED_BG,
 }
}

impl_theme! {
 SpinnerTheme: GuiControl::SPINNER => {
   text_inner_padding: u32 = GuiTextBoxProperty::TEXT_INNER_PADDING,
   text_lines_padding: u32 = GuiTextBoxProperty::TEXT_LINES_PADDING,
   color_selected_fg: Color = GuiTextBoxProperty::COLOR_SELECTED_FG,
   color_selected_bg: Color = GuiTextBoxProperty::COLOR_SELECTED_BG,
 }
}

impl_theme! {
 ScrollBarTheme: GuiControl::SCROLLBAR => {
   arrow_size: u32 = GuiScrollBarProperty::ARROWS_SIZE,
   arrow_visable: bool = GuiScrollBarProperty::ARROWS_VISIBLE,
   scroll_slider_padding: u32 = GuiScrollBarProperty::SCROLL_SLIDER_PADDING,
   scroll_slider_size: u32 = GuiScrollBarProperty::SCROLL_SLIDER_SIZE,
   scroll_padding: u32 = GuiScrollBarProperty::SCROLL_PADDING,
   scroll_speed: u32 = GuiScrollBarProperty::SCROLL_SPEED,
 }
}

impl_theme! {
 ListViewTheme: GuiControl::LISTVIEW => {
   list_item_height: u32 = GuiListViewProperty::LIST_ITEMS_HEIGHT,
   list_item_padding: u32 = GuiListViewProperty::LIST_ITEMS_PADDING,
   scrollbar_width: u32 = GuiListViewProperty::SCROLLBAR_WIDTH,
   scrollbar_side: GuiScrollBarSide = GuiListViewProperty::SCROLLBAR_SIDE,
 }
}

impl_theme! {
 ColorPickerTheme: GuiControl::COLORPICKER => {
   color_selector_size: u32 = GuiColorPickerProperty::COLOR_SELECTOR_SIZE,
   huebar_width: u32 = GuiColorPickerProperty::HUEBAR_WIDTH,
   huebar_padding: u32 = GuiColorPickerProperty::HUEBAR_PADDING,
   huebar_selector_height: u32 = GuiColorPickerProperty::HUEBAR_SELECTOR_HEIGHT,
   huebar_selector_overflow: u32 = GuiColorPickerProperty::HUEBAR_SELECTOR_OVERFLOW,
 }
}

impl_theme! {
  DefaultTheme(no_control_theme): GuiControl::DEFAULT => {
    text_size: u32 = GuiDefaultProperty::TEXT_SIZE => 18,
    text_spacing: u32 = GuiDefaultProperty::TEXT_SPACING,
    line_color: Color = GuiDefaultProperty::LINE_COLOR,
    background_color: Color = GuiDefaultProperty::BACKGROUND_COLOR,
  }
}

impl_theme! {
  ControlTheme(no_control_theme): GuiControl::DEFAULT => {
    normal: Color = GuiControlProperty::BASE_COLOR_NORMAL => rcolor(32, 32, 32, 255),
    focused: Color = GuiControlProperty::BASE_COLOR_FOCUSED => rcolor(64, 64, 64, 255),
    pressed: Color = GuiControlProperty::BASE_COLOR_PRESSED => rcolor(128, 128, 128, 255),
    disabled: Color = GuiControlProperty::BASE_COLOR_DISABLED => rcolor(16, 16, 16, 255),

    border_width: u32 = GuiControlProperty::BORDER_WIDTH => 2,
    border_normal: Color = GuiControlProperty::BORDER_COLOR_NORMAL => rcolor(128, 128, 128, 255),
    border_focused: Color = GuiControlProperty::BORDER_COLOR_FOCUSED => rcolor(192, 192, 192, 255),
    border_pressed: Color = GuiControlProperty::BORDER_COLOR_PRESSED => rcolor(224, 224, 224, 255),
    border_disabled: Color = GuiControlProperty::BORDER_COLOR_DISABLED => rcolor(24, 24, 24, 255),

    text_padding: u32 = GuiControlProperty::TEXT_PADDING,
    text_alignment: GuiTextAlignment = GuiControlProperty::TEXT_ALIGNMENT,

    text_normal: Color = GuiControlProperty::TEXT_COLOR_NORMAL => rcolor(192, 192, 192, 255),
    text_focused: Color = GuiControlProperty::TEXT_COLOR_FOCUSED => rcolor(210, 210, 210, 255),
    text_pressed: Color = GuiControlProperty::TEXT_COLOR_PRESSED => rcolor(255, 255, 255, 255),
    text_disabled: Color = GuiControlProperty::TEXT_COLOR_DISABLED => rcolor(128, 128, 128, 255),
  }
}
