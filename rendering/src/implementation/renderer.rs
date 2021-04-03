use crate::implementation::builder::color::ColorBuilder;
use crate::implementation::builder::texture::TextureBuilder;
use crate::implementation::shader::load_program;
use crate::interface::rendering::{AsciiRenderer, ColorRenderer, Renderer, TextureRenderer};
use crate::interface::{Color3f, Point2f, Size2d, TextureId};
use cgmath::ortho;
use glium::{Program, Surface};

const INDICES: glium::index::NoIndices =
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

struct TextureData {
    texture: glium::texture::Texture2d,
    builder: TextureBuilder,
}

pub struct GliumRenderer {
    size: Size2d,
    display: glium::Display,
    target: Option<glium::Frame>,
    color_builder: ColorBuilder,
    colored_program: Program,
    textured_program: Program,
    texture_data: Vec<TextureData>,
    matrix: cgmath::Matrix4<f32>,
}

impl GliumRenderer {
    pub fn new(
        display: glium::Display,
        textures: Vec<glium::texture::Texture2d>,
        size: Size2d,
    ) -> GliumRenderer {
        let colored_program = load_program(&display, "colored.vertex", "colored.fragment");
        let textured_program = load_program(&display, "textured.vertex", "textured.fragment");

        let matrix: cgmath::Matrix4<f32> = ortho(0.0, size.0 as f32, 0.0, size.1 as f32, -1.0, 1.0);

        let texture_data = textures
            .into_iter()
            .map(|texture| TextureData {
                texture,
                builder: TextureBuilder::new(16),
            })
            .collect();

        GliumRenderer {
            size,
            display,
            target: None,
            color_builder: ColorBuilder::default(),
            colored_program,
            textured_program,
            texture_data,
            matrix,
        }
    }

    fn render_colored_triangles(&mut self) {
        let target = self.target.as_mut().unwrap();
        let vertex_buffer =
            glium::VertexBuffer::new(&self.display, &self.color_builder.vertices).unwrap();

        let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(self.matrix)
        };

        target
            .draw(
                &vertex_buffer,
                &INDICES,
                &self.colored_program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    fn render_textured_triangles(&mut self) {
        let target = self.target.as_mut().unwrap();

        let draw_parameters = glium::draw_parameters::DrawParameters {
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..glium::draw_parameters::DrawParameters::default()
        };

        for data in &self.texture_data {
            let vertex_buffer =
                glium::VertexBuffer::new(&self.display, &data.builder.vertices).unwrap();

            let uniforms = uniform! {
                matrix: Into::<[[f32; 4]; 4]>::into(self.matrix),
                tex: &data.texture,
            };

            target
                .draw(
                    &vertex_buffer,
                    &INDICES,
                    &self.textured_program,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }
    }
}

impl Renderer for GliumRenderer {
    fn get_size(&self) -> Size2d {
        self.size
    }

    fn start(&mut self, color: Color3f) {
        let mut target = self.display.draw();
        target.clear_color(color.0, color.1, color.2, 1.0);
        self.target = Some(target);

        self.color_builder.vertices.clear();
        self.texture_data
            .iter_mut()
            .for_each(|x| x.builder.vertices.clear());
    }

    fn finish(&mut self) {
        self.render_colored_triangles();
        self.render_textured_triangles();

        if let Some(target) = self.target.take() {
            target.finish().unwrap();
        }
    }

    fn take_screenshot(&self, filename: &str) {
        let image: glium::texture::RawImage2d<u8> = self.display.read_front_buffer().unwrap();
        let image =
            image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned())
                .unwrap();
        let image = image::DynamicImage::ImageRgba8(image).flipv();
        image.save(filename).unwrap();
    }

    fn get_color_renderer(&mut self) -> &mut dyn ColorRenderer {
        &mut self.color_builder
    }

    fn get_texture_renderer(&mut self, id: TextureId) -> &mut dyn TextureRenderer {
        &mut self.texture_data[id].builder
    }

    fn get_ascii_renderer(&mut self, id: TextureId) -> &mut dyn AsciiRenderer {
        &mut self.texture_data[id].builder
    }
}

pub fn get_other_corners(position: Point2f, size: Point2f) -> [Point2f; 3] {
    let corner10 = (position.0 + size.0, position.1);
    let corner01 = (position.0, position.1 + size.1);
    let corner11 = (position.0 + size.0, position.1 + size.1);

    [corner10, corner01, corner11]
}
