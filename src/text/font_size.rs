use dioxus_native_core::{
    node_ref::{AttributeMask, NodeView},
    state::ParentDepState,
    NodeMask,
};
use lightningcss::{properties::font, traits::Parse, values::length::LengthPercentage};

#[derive(Clone, PartialEq, Debug)]
pub struct FontSize(pub f32);

impl Default for FontSize {
    fn default() -> Self {
        FontSize(16.0)
    }
}

impl ParentDepState for FontSize {
    type Ctx = ();
    type DepState = (Self,);
    const NODE_MASK: NodeMask = NodeMask::new_with_attrs(AttributeMask::Static(&["font-size"]));

    fn reduce(&mut self, node: NodeView<'_>, parent: Option<(&Self,)>, _: &Self::Ctx) -> bool {
        let new = if let Some(color_attr) = node.attributes().into_iter().flatten().next() {
            if let Some(as_text) = color_attr.value.as_text() {
                if let Ok(font_size) = font::FontSize::parse_string(as_text) {
                    font_size
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        };
        let parent = if let Some((parent,)) = parent {
            parent.0
        } else {
            16.0
        };
        let new = match new {
            font::FontSize::Length(length) => match length {
                LengthPercentage::Dimension(l) => match l {
                    lightningcss::values::length::LengthValue::Px(size) => size,
                    lightningcss::values::length::LengthValue::Em(size) => size * parent,
                    // TODO: this should be the font size of the root element
                    lightningcss::values::length::LengthValue::Rem(size) => size * 16.0,
                    _ => todo!(),
                },
                LengthPercentage::Percentage(percentage) => parent * percentage.0,
                _ => todo!(),
            },
            font::FontSize::Absolute(size) => match size {
                font::AbsoluteFontSize::XXSmall => 9.0,
                font::AbsoluteFontSize::XSmall => 10.0,
                font::AbsoluteFontSize::Small => 13.0,
                font::AbsoluteFontSize::Medium => 16.0,
                font::AbsoluteFontSize::Large => 18.0,
                font::AbsoluteFontSize::XLarge => 24.0,
                font::AbsoluteFontSize::XXLarge => 32.0,
            },
            font::FontSize::Relative(size) => match size {
                font::RelativeFontSize::Smaller => parent - 2.0,
                font::RelativeFontSize::Larger => parent + 2.0,
            },
        };

        if self.0 != new {
            *self = Self(new);
            true
        } else {
            false
        }
    }
}
