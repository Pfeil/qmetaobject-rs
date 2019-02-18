use qttypes::{QColor, QRectF};

cpp_class!(
/// Wrapper around QPen
#[derive(Default)]
pub unsafe struct QPen as "QPen");
impl QPen {
    // fn from_style
    pub fn from_color(color: QColor) -> Self {
        cpp!(unsafe [color as "QColor"] -> QPen as "QPen" {
            return QPen(color);
        })
    }
}


cpp_class!(
/// Wrapper around QPainter
#[derive(Default)]
pub unsafe struct QPainter as "QPainter");
impl QPainter {
    pub fn set_pen(&mut self, pen: QPen) {
        cpp!(unsafe [self as "QPainter*", pen as "QPen"] {
            self->setPen(pen);
        })
    }
    pub fn draw_pie_dimensions(&self, x: i32, y: i32, width: i32, height: i32, start_angle: i32, span_angle: i32) {
        cpp!(unsafe [self as "QPainter*", x as "int", y as "int", width as "int", height as "int", start_angle as "int", span_angle as "int"] {
            self->drawPie(x, y, height, width, start_angle, span_angle);
        });
    }
    pub fn draw_pie_rect(&self, rectangle: QRectF, start_angle: u32, span_angle: u32) {
        cpp!(unsafe [self as "QPainter*", rectangle as "QRectF", start_angle as "int", span_angle as "int"] {
            self->drawPie(rectangle, start_angle, span_angle);
        });
    }
}