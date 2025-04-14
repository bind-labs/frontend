use css_style::{prelude::*, StyleUpdater};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OverflowValue {
    Visible,
    Hidden,
    Scroll,
    AutoScroll,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Overflow {
    x: OverflowValue,
    y: OverflowValue,
}

impl Overflow {
    pub fn auto_scroll() -> Self {
        Self {
            x: OverflowValue::AutoScroll,
            y: OverflowValue::AutoScroll,
        }
    }

    pub fn hidden() -> Self {
        Self {
            x: OverflowValue::Hidden,
            y: OverflowValue::Hidden,
        }
    }

    pub fn visible() -> Self {
        Self {
            x: OverflowValue::Visible,
            y: OverflowValue::Visible,
        }
    }

    pub fn scroll() -> Self {
        Self {
            x: OverflowValue::Scroll,
            y: OverflowValue::Scroll,
        }
    }

    pub fn x_hidden(mut self) -> Self {
        self.x = OverflowValue::Hidden;
        self
    }

    pub fn x_visible(mut self) -> Self {
        self.x = OverflowValue::Visible;
        self
    }

    pub fn x_scroll(mut self) -> Self {
        self.x = OverflowValue::Scroll;
        self
    }

    pub fn x_auto_scroll(mut self) -> Self {
        self.x = OverflowValue::AutoScroll;
        self
    }

    pub fn y_hidden(mut self) -> Self {
        self.y = OverflowValue::Hidden;
        self
    }

    pub fn y_visible(mut self) -> Self {
        self.y = OverflowValue::Visible;
        self
    }

    pub fn y_scroll(mut self) -> Self {
        self.y = OverflowValue::Scroll;
        self
    }

    pub fn y_auto_scroll(mut self) -> Self {
        self.y = OverflowValue::AutoScroll;
        self
    }
}

impl StyleUpdater for Overflow {
    fn update_style(self, style: Style) -> Style {
        let get_val = |val| match val {
            OverflowValue::Visible => "visible",
            OverflowValue::Hidden => "hidden",
            OverflowValue::Scroll => "scroll",
            OverflowValue::AutoScroll => "auto",
        };
        style
            .insert("overflow-x", get_val(self.x))
            .insert("overflow-y", get_val(self.y))
    }
}
