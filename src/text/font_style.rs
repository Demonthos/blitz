use dioxus_native_core::{
    node::{OwnedAttributeValue, OwnedAttributeView},
    node_ref::{AttributeMaskBuilder, NodeMaskBuilder, NodeView},
    Dependancy, Pass, SendAnyMap,
};
use lightningcss::traits::Parse;
use lightningcss::{
    properties::font::{
        AbsoluteFontSize, FontSize, FontStretch, FontStyle, FontVariantCaps, FontWeight,
        GenericFontFamily, LineHeight, RelativeFontSize,
    },
    values::{length::LengthValue, percentage::DimensionPercentage},
};

use crate::util::Resolve;

pub const DEFAULT_FONT_SIZE: ComputedFontSize = ComputedFontSize(16.0);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ComputedFontSize(pub f32);

impl ComputedFontSize {
    pub fn compute_from(&self, font_size: FontSize, parent_font_size: ComputedFontSize) -> Self {
        match font_size {
            FontSize::Length(length) => {
                Self(length.resolve(parent_font_size.0, parent_font_size, viewport_size))
            }
            FontSize::Absolute(abs_val) => {
                let factor = match abs_val {
                    AbsoluteFontSize::XXSmall => 0.6,
                    AbsoluteFontSize::XSmall => 0.75,
                    AbsoluteFontSize::Small => 0.89, // 8/9
                    AbsoluteFontSize::Medium => 1.0,
                    AbsoluteFontSize::Large => 1.25,
                    AbsoluteFontSize::XLarge => 1.5,
                    AbsoluteFontSize::XXLarge => 2.0,
                };
                Self(factor * DEFAULT_FONT_SIZE.0)
            }
            FontSize::Relative(rel_val) => {
                let factor = match rel_val {
                    RelativeFontSize::Smaller => 0.8,
                    RelativeFontSize::Larger => 1.25,
                };
                Self(factor * parent_font_size.0)
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Font {
    pub family: Vec<OwnedFontFamily>,
    pub size: ComputedFontSize,
    pub style: FontStyle,
    pub weight: FontWeight,
    pub stretch: FontStretch,
    pub line_height: LineHeight,
    pub variant_caps: FontVariantCaps,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: vec![OwnedFontFamily::default()],
            size: DEFAULT_FONT_SIZE,
            style: FontStyle::default(),
            weight: FontWeight::default(),
            stretch: FontStretch::default(),
            line_height: LineHeight::default(),
            variant_caps: FontVariantCaps::default(),
        }
    }
}

impl Pass for Font {
    type ParentDependencies = ();
    type ChildDependencies = ();
    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[
            "font",
            "font-family",
            "font-size",
            "font-size-adjust",
            "font-stretch",
            "font-style",
            "font-variant",
            "font-weight",
        ]))
        .with_element();

    fn pass<'a>(
        &mut self,
        node_view: NodeView,
        _: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _: Option<Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>>,
        _: &SendAnyMap,
    ) -> bool {
        let mut new = Self::default();

        // handle text modifier elements
        if node_view.namespace().is_none() {
            if let Some(tag) = node_view.tag() {
                match tag {
                    // "b" => apply_style_attributes("font-weight", "bold", &mut new),
                    // "strong" => apply_style_attributes("font-weight", "bold", &mut new),
                    // "i" => apply_style_attributes("font-style", "italic", &mut new),
                    // "em" => apply_style_attributes("font-style", "italic", &mut new),
                    // "mark" => {
                    //     apply_style_attributes("background-color", "rgba(241, 231, 64, 50%)", self)
                    // }
                    _ => (),
                }
            }
        }

        // gather up all the styles from the attribute list
        if let Some(attrs) = node_view.attributes() {
            for OwnedAttributeView {
                attribute, value, ..
            } in attrs
            {
                match attribute.name.as_str() {
                    _ => unreachable!(),
                }
            }
        }

        if &mut new != self {
            *self = new;
            true
        } else {
            false
        }
    }

    fn create<'a>(
        node_view: NodeView<()>,
        node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        children: Option<Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>>,
        context: &SendAnyMap,
    ) -> Self {
        let mut myself = Self::default();
        myself.pass(node_view, node, parent, children, context);
        myself
    }
}

#[derive(Clone, PartialEq, Debug)]
enum OwnedFontFamily {
    Generic(GenericFontFamily),
    FamilyName(String),
}

impl Default for OwnedFontFamily {
    fn default() -> Self {
        Self::Generic(GenericFontFamily::Default)
    }
}

fn parse_font_size_from_attr(
    css_value: &OwnedAttributeValue,
    parent_font_size: f32,
    root_font_size: f32,
) -> Option<f32> {
    match css_value {
        OwnedAttributeValue::Text(n) => {
            // css font-size parse.
            // not support
            // 1. calc,
            // 3. relative font size. (smaller, larger)
            match FontSize::parse_string(n) {
                Ok(FontSize::Length(length)) => match length {
                    DimensionPercentage::Dimension(l) => match l {
                        LengthValue::Rem(v) => Some(v * root_font_size),
                        LengthValue::Em(v) => Some(v * parent_font_size),
                        _ => l.to_px(),
                    },
                    // same with em.
                    DimensionPercentage::Percentage(p) => Some(p.0 * parent_font_size),
                    DimensionPercentage::Calc(_c) => None,
                },
                Ok(FontSize::Absolute(abs_val)) => {
                    let factor = match abs_val {
                        AbsoluteFontSize::XXSmall => 0.6,
                        AbsoluteFontSize::XSmall => 0.75,
                        AbsoluteFontSize::Small => 0.89, // 8/9
                        AbsoluteFontSize::Medium => 1.0,
                        AbsoluteFontSize::Large => 1.25,
                        AbsoluteFontSize::XLarge => 1.5,
                        AbsoluteFontSize::XXLarge => 2.0,
                    };
                    Some(factor * root_font_size)
                }
                Ok(FontSize::Relative(rel_val)) => {
                    let factor = match rel_val {
                        RelativeFontSize::Smaller => 0.8,
                        RelativeFontSize::Larger => 1.25,
                    };
                    Some(factor * parent_font_size)
                }
                _ => None,
            }
        }
        OwnedAttributeValue::Float(n) => Some(n.to_owned() as f32),
        OwnedAttributeValue::Int(n) => Some(n.to_owned() as f32),
        _ => None,
    }
}
