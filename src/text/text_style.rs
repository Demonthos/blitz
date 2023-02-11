use dioxus_native_core::{
    node::{OwnedAttributeValue, OwnedAttributeView},
    node_ref::{AttributeMaskBuilder, NodeMaskBuilder, NodeView},
    Dependancy, Pass, SendAnyMap,
};
use lightningcss::values::color::CssColor;
use lightningcss::{
    properties::text::{TextDecorationLine, TextDecorationStyle, TextDecorationThickness},
    traits::Parse,
};

/// https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration
#[derive(Default, Clone, PartialEq, Debug)]
pub struct TextDecoration {
    pub line: TextDecorationLine,
    pub thickness: TextDecorationThickness,
    pub style: TextDecorationStyle,
    pub color: CssColor,
}

impl Pass for TextDecoration {
    type ParentDependencies = ();
    type ChildDependencies = ();
    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[
            "text-decoration",
            "text-decoration-line",
            "text-decoration-color",
            "text-decoration-style",
            "text-decoration-thickness",
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
                    "ins" | "u" => new.line.insert(TextDecorationLine::Underline),
                    "del" => new.line.insert(TextDecorationLine::LineThrough),
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
                if let OwnedAttributeValue::Text(txt) = value {
                    match attribute.name.as_str() {
                        "text-decoration" => {
                            if let Ok(value) =
                                lightningcss::properties::text::TextDecoration::parse_string(txt)
                            {
                                new.line = value.line;
                                new.style = value.style;
                                new.thickness = value.thickness;
                                new.color = value.color;
                            }
                        }
                        "text-decoration-line" => {
                            if let Ok(value) =
                                lightningcss::properties::text::TextDecorationLine::parse_string(
                                    txt,
                                )
                            {
                                new.line = value;
                            }
                        }
                        "text-decoration-color" => {
                            if let Ok(value) = CssColor::parse_string(txt) {
                                new.color = value;
                            }
                        }
                        "text-decoration-style" => {
                            if let Ok(value) = TextDecorationStyle::parse_string(txt) {
                                new.style = value;
                            }
                        }
                        "text-decoration-thickness" => {
                            if let Ok(value) = TextDecorationThickness::parse_string(txt) {
                                new.thickness = value;
                            }
                        }
                        _ => unreachable!(),
                    }
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
