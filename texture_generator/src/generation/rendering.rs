use crate::generation::RuntimeData;
use crate::math::aabb::AABB;
use crate::math::color::Color;
use crate::math::shape::Shape;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RenderComponent {
    Shape { shape: Shape, color: Color },
}

impl RenderComponent {
    pub fn new_shape(shape: Shape, color: Color) -> RenderComponent {
        RenderComponent::Shape { shape, color }
    }

    pub fn render(&self, data: &mut dyn RuntimeData, aabb: &AABB) {
        match self {
            RenderComponent::Shape { shape, color } => {
                let mut point = aabb.start();

                while point.y < aabb.end().y {
                    point.x = aabb.start().x;

                    while point.x < aabb.end().x {
                        if shape.is_inside(&point) {
                            data.set(&point, color);
                        }

                        point.x += 1;
                    }

                    point.y += 1;
                }
            }
        }
    }
}
