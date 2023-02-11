use lightningcss::properties::border::BorderSideWidth;
use lightningcss::values;
use taffy::prelude::Size;
use values::calc::{Calc, MathFunction};
use values::color::CssColor;
use values::length::{Length, LengthValue};
use values::percentage::DimensionPercentage;
use vello::peniko::Color;

use crate::text::font_style::{ComputedFontSize, DEFAULT_FONT_SIZE};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) enum Axis {
    X,
    Y,
    // the smallest axis
    Min,
    // the largest axis
    Max,
}

pub(crate) fn translate_color(color: &CssColor) -> Color {
    let rgb = color.to_rgb();
    if let CssColor::RGBA(rgba) = rgb {
        Color::rgba(
            rgba.red as f64 / 255.0,
            rgba.green as f64 / 255.0,
            rgba.blue as f64 / 255.0,
            rgba.alpha as f64 / 255.0,
        )
    } else {
        panic!("translation failed");
    }
}

pub(crate) trait Resolve {
    fn resolve(
        &self,
        container_size: f32,
        font_size: ComputedFontSize,
        viewport_size: &Size<u32>,
    ) -> f32;
}

impl<T: Resolve> Resolve for Calc<T> {
    fn resolve(
        &self,
        container_size: f32,
        font_size: ComputedFontSize,
        viewport_size: &Size<u32>,
    ) -> f32 {
        match self {
            values::calc::Calc::Value(v) => v.resolve(container_size, font_size, viewport_size),
            values::calc::Calc::Number(px) => *px,
            values::calc::Calc::Sum(v1, v2) => {
                v1.resolve(container_size, font_size, viewport_size)
                    + v2.resolve(container_size, font_size, viewport_size)
            }
            values::calc::Calc::Product(v1, v2) => {
                *v1 * v2.resolve(container_size, font_size, viewport_size)
            }
            values::calc::Calc::Function(f) => f.resolve(container_size, font_size, viewport_size),
        }
    }
}

impl<T: Resolve> Resolve for MathFunction<T> {
    fn resolve(
        &self,
        container_size: f32,
        font_size: ComputedFontSize,
        viewport_size: &Size<u32>,
    ) -> f32 {
        match self {
            values::calc::MathFunction::Calc(c) => {
                c.resolve(container_size, font_size, viewport_size)
            }
            values::calc::MathFunction::Min(v) => v
                .iter()
                .map(|v| v.resolve(container_size, font_size, viewport_size))
                .min_by(|f1, f2| f1.partial_cmp(f2).unwrap())
                .unwrap(),
            values::calc::MathFunction::Max(v) => v
                .iter()
                .map(|v| v.resolve(container_size, font_size, viewport_size))
                .max_by(|f1, f2| f1.partial_cmp(f2).unwrap())
                .unwrap(),
            values::calc::MathFunction::Clamp(min, val, max) => {
                min.resolve(container_size, font_size, viewport_size).max(
                    val.resolve(container_size, font_size, viewport_size)
                        .min(max.resolve(container_size, font_size, viewport_size)),
                )
            }
            _ => todo!(),
        }
    }
}

impl Resolve for BorderSideWidth {
    fn resolve(
        &self,
        container_size: f32,
        font_size: ComputedFontSize,
        viewport_size: &Size<u32>,
    ) -> f32 {
        match self {
            BorderSideWidth::Thin => 2.0,
            BorderSideWidth::Medium => 4.0,
            BorderSideWidth::Thick => 6.0,
            BorderSideWidth::Length(l) => l.resolve(container_size, font_size, viewport_size),
        }
    }
}

impl Resolve for LengthValue {
    fn resolve(
        &self,
        container_size: f32,
        font_size: ComputedFontSize,
        viewport_size: &Size<u32>,
    ) -> f32 {
        use values::length::LengthValue::*;
        match self {
            Px(px) => *px,
            Vw(vw) => *vw * viewport_size.width as f32 / 100.0,
            Vh(vh) => *vh * viewport_size.height as f32 / 100.0,
            Vmin(vmin) => *vmin * viewport_size.height.min(viewport_size.width) as f32 / 100.0,
            Vmax(vmax) => *vmax * viewport_size.height.max(viewport_size.width) as f32 / 100.0,
            Rem(v) => v * font_size.0,
            Em(v) => v * DEFAULT_FONT_SIZE.0,
            _ => self.to_px().expect("handle more unit conversions"),
        }
    }
}

impl Resolve for Length {
    fn resolve(
        &self,
        container_size: f32,
        font_size: ComputedFontSize,
        viewport_size: &Size<u32>,
    ) -> f32 {
        match self {
            Length::Value(l) => l.resolve(container_size, font_size, viewport_size),
            Length::Calc(c) => c.resolve(container_size, font_size, viewport_size),
        }
    }
}

impl<T: Resolve> Resolve for DimensionPercentage<T> {
    fn resolve(
        &self,
        container_size: f32,
        font_size: ComputedFontSize,
        viewport_size: &Size<u32>,
    ) -> f32 {
        match self {
            DimensionPercentage::Dimension(v) => {
                v.resolve(container_size, font_size, viewport_size)
            }
            DimensionPercentage::Percentage(p) => container_size * p.0,
            DimensionPercentage::Calc(c) => c.resolve(container_size, font_size, viewport_size),
        }
    }
}

pub fn axis_size(axis: Axis, rect: &Size<f32>) -> f32 {
    match axis {
        Axis::X => rect.width,
        Axis::Y => rect.height,
        Axis::Min => rect.width.min(rect.height),
        Axis::Max => rect.width.max(rect.height),
    }
}
