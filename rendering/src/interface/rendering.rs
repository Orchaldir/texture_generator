use crate::interface::{Color3f, Point2f, Size2d, TextureCoordinate, TextureId};

/// A trait to load & init resources for rendering during initialization.
pub trait Initialization {
    /// Loads a texture from a file and returns a `TextureId` as a handle.
    ///
    /// # Panics
    ///
    /// Panics if the file does not exist.
    ///
    /// Panics if the file is not an image.
    ///
    /// Panics if it can not create a texture from the image.
    fn load_texture(&mut self, filename: &str) -> TextureId;
}

/// A trait to abstract away different rendering libraries and render targets.
pub trait Renderer {
    /// Returns the size of the render target in tiles.
    /// A tile is big enough to hold a single ascii character.
    fn get_size(&self) -> Size2d;

    /// Starts the rendering and fills the render target with the Color `color`.
    fn start(&mut self, color: Color3f);

    /// Finishes the rendering.
    fn finish(&mut self);

    /// Takes a screenshot and saves it.
    fn take_screenshot(&self, filename: &str);

    /// Gets a renderer for colored polygons.
    fn get_color_renderer(&mut self) -> &mut dyn ColorRenderer;

    /// Gets a renderer for textured polygons.
    fn get_texture_renderer(&mut self, id: TextureId) -> &mut dyn TextureRenderer;

    /// Gets a renderer for text.
    fn get_ascii_renderer(&mut self, id: TextureId) -> &mut dyn AsciiRenderer;
}

/// A trait that focuses on rendering colored polygons.
pub trait ColorRenderer {
    #[svgbobdoc::transform]
    /// Renders the triangle defined by the points a, b & c.
    ///
    /// The points must be in counter-clockwise order:
    /// ```svgbob
    ///    c
    ///    *
    ///   / \
    ///  /   \
    /// *-----*
    /// a     b
    /// ```
    fn render_triangle(&mut self, a: Point2f, b: Point2f, c: Point2f, color: Color3f);

    /// Renders an axis-aligned rectangle.
    fn render_rectangle(&mut self, position: Point2f, size: Point2f, color: Color3f);
}

/// A trait that focuses on rendering textured polygons.
pub trait TextureRenderer {
    /// Renders an axis-aligned textured rectangle.
    ///
    /// The parameters tc & tc_size define an axis-aligned rectangle inside the texture.
    fn render_rectangle(
        &mut self,
        position: Point2f,
        size: Point2f,
        tc: TextureCoordinate,
        tc_size: TextureCoordinate,
        color: Color3f,
    );
}

/// A trait that focuses on rendering text.
pub trait AsciiRenderer {
    /// Renders a whole string.
    fn render_text(&mut self, position: Point2f, size: Point2f, string: &str, color: Color3f);

    /// Renders an unicode character, if it is ascii.
    fn render_char(&mut self, position: Point2f, size: Point2f, character: char, color: Color3f);

    /// Renders an ascii character.
    fn render_u8(&mut self, position: Point2f, size: Point2f, ascii: u8, color: Color3f);
}
