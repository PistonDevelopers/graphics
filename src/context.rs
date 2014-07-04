
use add;
use {
    BackEnd,
    BevelBorderLineColorContext,
    BevelRectangleColorContext,
    BevelRectangleBorderColorContext,
    ColorContext,
    Draw,
    EllipseColorContext,
    EllipseBorderColorContext,
    ImageSize,
    ImageContext,
    ImageColorContext,
    ImageRectangleContext,
    ImageRectangleColorContext,
    LerpTweenPolygonsColorContext,
    PolygonColorContext,
    RectangleContext,
    RectangleColorContext,
    RectangleBorderColorContext,
    RoundBorderLineColorContext,
    RoundRectangleColorContext,
    RoundRectangleBorderColorContext,
    SquareBorderLineColorContext,
};
use triangulation;
use internal::{
    CanColor,
    CanRectangle,
    CanSourceRectangle,
    CanTransform,
    CanViewTransform,
    Color,
    ColorComponent,
    HasColor,
    HasRectangle,
    HasSourceRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Radius,
    Rectangle,
    Polygon,
    Polygons,
    Scalar,
};
use shape;

/// Drawing 2d context.
#[deriving(Copy)]
pub struct Context<S=(), C=()> {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    shape: S,
    color: C,
}

impl<S: Copy, C: Copy>
Clone 
for Context<S, C> {
    #[inline(always)]
    fn clone(&self) -> Context<S, C> {
        *self 
    }
}

impl<S, C>
HasTransform<Matrix2d> 
for Context<S, C> {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl<S: Copy, C: Copy>
CanTransform<Context<S, C>, Matrix2d> 
for Context<S, C> {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> Context<S, C> {
        Context {
            transform: value,
            ..*self
        }
    }
}

impl<S, C>
HasViewTransform<Matrix2d> 
for Context<S, C> {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl<S: Copy, C: Copy>
CanViewTransform<Context<S, C>, Matrix2d> 
for Context<S, C> {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> Context<S, C> {
        Context { view: value, ..*self }
    }
}

impl Context {
    /// Creates a new drawing context.
    #[inline(always)]
    pub fn new() -> Context {
        Context {
            view:
                [1.0, 0.0, 0.0,
                 0.0, 1.0, 0.0]
            ,
            transform:
                [1.0, 0.0, 0.0,
                 0.0, 1.0, 0.0]
            ,
            shape: (),
            color: (),
        }
    }

    /// Creates a new drawing context in absolute coordinates.
    ///
    /// This function assumes the default coordinate system
    /// being centered with x axis pointing to the right
    /// and y axis pointing up.
    ///
    /// Returns a drawing context
    /// with origin in the upper left corner
    /// and x axis pointing to the right
    /// and y axis pointing down.
    #[inline(always)]
    pub fn abs(w: f64, h: f64) -> Context {
        let sx = 2.0 / w;
        let sy = -2.0 / h;
        let mat = [ sx, 0.0, -1.0,
                   0.0,  sy, 1.0 ];
        Context {
            view: mat,
            transform: mat,
            shape: (),
            color: (),
        }
    }
}

#[test]
fn test_context() {
    use RelativeTransform2d;

    let c = Context::new();
    {
        let d = c.trans(20.0, 40.0);
        let d = d.trans(10.0, 10.0);
        let transform = d.transform;
        assert_eq!(transform[2], 30.0);
        assert_eq!(transform[5], 50.0);
    }
    
    let transform = c.transform;
    assert_eq!(transform[2], 0.0);
    assert_eq!(transform[5], 0.0);

    let c = c.rot_deg(90.0);
    let transform = c.transform;
    assert!((transform[0] - 0.0).abs() < 0.00001);
    assert!((transform[1] + 1.0).abs() < 0.00001);
}

#[test]
fn test_scale() {
    use RelativeTransform2d;

    let c = Context::new();
    let c = c.scale(2.0, 3.0);
    let transform = c.transform;
    assert!((transform[0] - 2.0).abs() < 0.00001);
    assert!((transform[4] - 3.0).abs() < 0.00001);
}

impl<C: Copy>
add::AddRectangle<Context<shape::RectangleShape, C>> 
for Context<(), C> {
    #[inline(always)]
    fn rect(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> Context<shape::RectangleShape, C> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape { 
                variant: shape::RectangleVariant([x, y, w, h]),
                border_radius: (),
                corner: (),
            },
        }
    }
}

impl<'b, I> 
add::AddRectangle<ImageRectangleColorContext<'b, I>> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn rect(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> ImageRectangleColorContext<'b, I> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape {
                    variant: shape::ImageVariant {
                            image: self.shape.variant.image,
                            src_rect: self.shape.variant.src_rect,
                            rect: [x, y, w, h],
                        },
                    border_radius: self.shape.border_radius,
                    corner: self.shape.corner,
                },
        }
    }
}

#[test]
fn test_rect() {
    use add::AddRectangle;    

    let c = Context::new();
    let d = c.rect(0.0, 0.0, 100.0, 50.0);
    let shape::RectangleVariant(rect) = d.shape.variant;
    assert_eq!(rect[2], 100.0);
}

impl<S: Copy>
add::AddColor<Context<S, Color>> 
for Context<S> {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> Context<S, Color> {
        Context {
            view: self.view,
            transform: self.transform,
            shape: self.shape,
            color: [r, g, b, a],
        }
    }
}

#[test]
fn test_rgba() {
    use add::AddColor;    

    let c = Context::new();
    let d: ColorContext = c.rgba(1.0, 0.0, 0.0, 1.0);
    let color = d.color;
    assert_eq!(color[0], 1.0);
}

impl<C: Copy>
add::AddEllipse<Context<shape::EllipseShape, C>> 
for Context<(), C> {
    #[inline(always)]
    fn ellipse(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> Context<shape::EllipseShape, C> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape { 
                variant: shape::EllipseVariant([x, y, w, h]),
                border_radius: (),
                corner: (),
            },
        }
    }
}

#[test]
fn test_ellipse() {
    use add::AddEllipse;    

    let c = Context::new();
    let d: EllipseContext = c.ellipse(0.0, 0.0, 100.0, 100.0);
    let shape::EllipseVariant(rect) = d.shape.variant;    
    assert_eq!(rect[2], 100.0);
}

impl<'b, C: Copy> 
add::AddPolygon<'b, Context<Polygon<'b>, C>> 
for Context<(), C> {
    #[inline(always)]
    fn polygon<'b>(
        &self, 
        polygon: Polygon<'b>
    ) -> Context<Polygon<'b>, C> {
        Context {
            view: self.view,
            transform: self.transform,
            shape: polygon,
            color: self.color,
        }
    }
}

impl<'b, I: ImageSize, C: Copy> 
add::AddImage<'b, Context<shape::ImageShape<'b, I>, C>, I> 
for Context<(), C> {
    #[inline(always)]
    fn image(&self, image: &'b I) -> Context<shape::ImageShape<'b, I>, C> {
        let (w, h) = image.get_size();
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape {
                    variant: shape::ImageVariant {
                            image: image,
                            src_rect: [0, 0, w as i32, h as i32],
                            rect: (),
                        },
                    border_radius: (),
                    corner: (),
                },
        }
    }
}

impl<S: HasRectangle<Rectangle>, C>
HasRectangle<Rectangle> 
for Context<S, C> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.shape.get_rectangle()
    }
}

impl<S: HasSourceRectangle<Rectangle>, C>
HasSourceRectangle<Rectangle> 
for Context<S, C> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> Rectangle {
        self.shape.get_source_rectangle()
    }
}

impl<'b, I: ImageSize> 
add::AddImage<'b, ImageRectangleContext<'b, I>, I> 
for RectangleContext {
    fn image(
        &self, 
        image: &'b I
    ) -> ImageRectangleContext<'b, I> {
        let (w, h) = image.get_size();
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape {
                    variant: shape::ImageVariant {
                            image: image,
                            rect: self.shape.get_rectangle(),
                            src_rect: [0, 0, w as i32, h as i32],
                        },
                    border_radius: self.shape.border_radius,
                    corner: self.shape.corner,
                },
        }
    }
}

impl<S: add::AddRound<S2>, S2: Copy, C: Copy>
add::AddRound<Context<S2, C>> 
for Context<S, C> {
    #[inline(always)]
    fn round(
        &self, 
        radius: Radius
    ) -> Context<S2, C> {
        Context {
            view: self.view,
            transform: self.transform,
            shape: self.shape.round(radius),
            color: self.color,
        }
    }
}

impl<S: add::AddBorder<S2>, S2: Copy, C: Copy>
add::AddBorder<Context<S2, C>> 
for Context<S, C> {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: Radius
    ) -> Context<S2, C> {
        Context {
            view: self.view,
            transform: self.transform,
            shape: self.shape.border_radius(radius),
            color: self.color,
        }
    }
}

impl<'a, S: add::AddPolygons<'a, S2>, S2: Copy, C: Copy>
add::AddPolygons<'a, Context<S2, C>> 
for Context<S, C> {
    #[inline(always)]
    fn polygons(
        &self, 
        polygons: Polygons<'a>
    ) -> Context<S2, C> {
        Context {
            view: self.view,
            transform: self.transform,
            shape: self.shape.polygons(polygons),
            color: self.color,
        }
    }
}

impl<S: Copy + CanRectangle<S, Rectangle>, C: Copy>
CanRectangle<Context<S, C>, Rectangle> 
for Context<S, C> {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> Context<S, C> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: self.shape.rectangle(rect),
        }
    }
}

impl<S: Copy + CanSourceRectangle<S, Rectangle>, C: Copy>
CanSourceRectangle<Context<S, C>, Rectangle> 
for Context<S, C> {
    #[inline(always)]
    fn source_rectangle(
        &self, 
        rect: Rectangle
    ) -> Context<S, C> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: self.shape.source_rectangle(rect),
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for PolygonColorContext<'b> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygon = self.shape;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_polygon_tri_list_xy_f32_rgba_f32(
                self.transform,
                polygon,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<S> 
HasColor<Color> 
for Context<S, Color> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

// If the context has no color, use the color of the shape.
impl<S: HasColor<Color>>
HasColor<Color>
for Context<S> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.shape.get_color()
    }
}

impl<S: Copy> 
CanColor<Context<S, Color>, Color> 
for Context<S, Color> {
    #[inline(always)]
    fn color(&self, value: Color) -> Context<S, Color> {
        Context {
            view: self.view,
            transform: self.transform,
            color: value,
            shape: self.shape,
        }
    }
}

#[test]
fn test_rgba_1() {
    use add::AddRectangle;
    use add::AddColor;

    let c = Context::new();
    let d = c.rect(0.0, 0.0, 100.0, 100.0);
    let e = d.rgba(1.0, 0.0, 0.0, 1.0);
    let color = e.color;
    assert_eq!(color[0], 1.0);
}

impl<C: Copy>
add::AddLine<Context<shape::LineShape, C>> 
for Context<(), C> {
    #[inline(always)]
    fn line(
        &self, 
        x1: Scalar, 
        y1: Scalar, 
        x2: Scalar, 
        y2: Scalar
    ) -> Context<shape::LineShape, C> {
        Context {
            view: self.view,
            transform: self.transform,
            shape: shape::Shape {
                    variant: shape::LineVariant([x1, y1, x2, y2]),
                    border_radius: (),
                    corner: (),
                },
            color: self.color,
        }
    }
}

#[test]
fn test_line() {
    use add::AddLine;
    use add::AddColor;    

    let _c = Context::new().line(0.0, 0.0, 1.0, 1.0).rgb(1.0, 0.0, 0.0);
    let _c = Context::new().rgb(1.0, 0.0, 0.0).line(0.0, 0.0, 1.0, 1.0);
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let color: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];
            let shape::ImageVariant { 
                    image: texture,
                    src_rect: source_rect,
                    rect: (),
                } = self.shape.variant;
            let rect = [
                0.0, 
                0.0, 
                source_rect[2] as f64, 
                source_rect[3] as f64
            ];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque 
            // or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 
                || back_end.has_texture_alpha(texture);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                triangulation::rect_tri_list_xy_f32(self.transform, rect),
                triangulation::rect_tri_list_rgba_f32(color),
                triangulation::rect_tri_list_uv_f32(texture, source_rect)
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ColorContext {
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color;
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

impl<C: Copy>
add::AddTween<Context<shape::LerpTweenShape, C>> 
for Context<(), C> {
    #[inline(always)]
    fn lerp(
        &self, 
        tween_factor: Scalar
    ) -> Context<shape::LerpTweenShape, C> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape {
                    variant: shape::LerpTweenVariant {
                            lerp: tween_factor,
                            shapes: (),
                        },
                    border_radius: (),
                    corner: (),
                },
        }
    }
}

impl<C: Copy>
add::AddSquareBorder<Context<shape::SquareBorderLineShape, C>> 
for Context<shape::LineShape, C> {
    #[inline(always)]
    fn square_border_radius(
        &self, 
        radius: Radius
    ) -> Context<shape::SquareBorderLineShape, C> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape {
                    corner: shape::SquareCorner(radius),
                    variant: self.shape.variant,
                    border_radius: self.shape.border_radius,
                },
        }
    }
}


impl<C: Copy>
add::AddBevelBorder<Context<shape::BevelBorderLineShape, C>> 
for Context<shape::LineShape, C> {
    #[inline(always)]
    fn bevel_border_radius(
        &self, 
        radius: Radius
    ) -> Context<shape::BevelBorderLineShape, C> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape {
                    corner: shape::BevelCorner(radius),
                    variant: self.shape.variant,
                    border_radius: self.shape.border_radius,
                },
        }
    }
}

impl<C: Copy>
add::AddRoundBorder<Context<shape::RoundBorderLineShape, C>> 
for Context<shape::LineShape, C> {
    #[inline(always)]
    fn round_border_radius(
        &self, 
        radius: Radius
    ) -> Context<shape::RoundBorderLineShape, C> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape {
                    corner: shape::RoundCorner(radius),
                    variant: self.shape.variant,
                    border_radius: self.shape.border_radius,
                },
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for EllipseColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.shape.get_rectangle();
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_ellipse_tri_list_xy_f32_rgba_f32(
                128,
                self.transform,
                rect,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'b, I> 
add::AddRectangle<ImageRectangleContext<'b, I>> 
for ImageContext<'b, I> {
    #[inline(always)]
    fn rect(
        &self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> ImageRectangleContext<'b, I> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape {
                variant: shape::ImageVariant {
                    image: self.shape.variant.image,
                    src_rect: self.shape.variant.src_rect,
                    rect: [x, y, w, h],
                },
                border_radius: (),
                corner: (),
            }
        }
    }
}

impl<'b, I: ImageSize> 
add::AddImage<'b, ImageRectangleColorContext<'b, I>, I> 
for RectangleColorContext {
    fn image(
        &self, 
        image: &'b I
    ) -> ImageRectangleColorContext<'b, I> {
        let (w, h) = image.get_size();
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: shape::Shape {
                    variant: shape::ImageVariant {
                            image: image,
                            src_rect: [0, 0, w as i32, h as i32],
                            rect: self.shape.get_rectangle(),
                        },
                    border_radius: self.shape.border_radius,
                    corner: self.shape.corner,
                },
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RectangleBorderColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.shape.get_rectangle();
            let color = self.color;
            let border_radius = self.shape.border_radius;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.tri_list_xy_f32_rgba_f32(
                triangulation::rect_border_tri_list_xy_f32(
                    self.transform, rect, border_radius),
                triangulation::rect_border_tri_list_rgba_f32(color)
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<S: Copy + add::AddBevel<T>, T, C: Copy>
add::AddBevel<Context<T, C>> 
for Context<S, C> {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: f64
    ) -> Context<T, C> {
        Context {
            view: self.view,
            transform: self.transform,
            color: self.color,
            shape: self.shape.bevel(radius),
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RectangleColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let rect = self.shape.get_rectangle();
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.tri_list_xy_f32_rgba_f32(
                triangulation::rect_tri_list_xy_f32(self.transform, rect),
                triangulation::rect_tri_list_rgba_f32(color)
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RoundBorderLineColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let shape::LineVariant(line) = self.shape.variant;
            let shape::RoundCorner(round_border_radius) = self.shape.corner;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_round_border_line_tri_list_xy_f32_rgba_f32(
                64,
                self.transform,
                line,
                round_border_radius,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for LerpTweenPolygonsColorContext<'b> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let shape::Shape {
                    variant: shape::LerpTweenVariant {
                            lerp: tween_factor,
                            shapes: polygons,
                        },
                    border_radius: (),
                    corner: (),
                } = self.shape;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_lerp_polygons_tri_list_xy_f32_rgba_f32(
                self.transform,
                polygons,
                tween_factor,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ImageColorContext<'b, I> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let color = self.color;
            let shape::Shape {
                    variant: shape::ImageVariant {
                            image: texture,
                            src_rect: source_rect,
                            rect: ()
                        },
                    border_radius: (),
                    corner: ()
                } = self.shape;
            let rect = [
                0.0, 
                0.0, 
                source_rect[2] as f64, 
                source_rect[3] as f64
            ];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely
            // opaque or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 
                || back_end.has_texture_alpha(texture);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                triangulation::rect_tri_list_xy_f32(self.transform, rect),
                triangulation::rect_tri_list_rgba_f32(color),
                triangulation::rect_tri_list_uv_f32(texture, source_rect)
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ImageRectangleContext<'b, I> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let color: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];
            let shape::Shape {
                    variant: shape::ImageVariant {
                            image: texture,
                            src_rect: source_rect,
                            rect: rect
                        },
                    border_radius: (),
                    corner: ()
                } = self.shape;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque 
            // or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 
                || back_end.has_texture_alpha(texture);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                triangulation::rect_tri_list_xy_f32(self.transform, rect),
                triangulation::rect_tri_list_rgba_f32(color),
                triangulation::rect_tri_list_uv_f32(texture, source_rect)
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'b, B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for ImageRectangleColorContext<'b, I> {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let color = self.color;
            let shape::Shape {
                    variant: shape::ImageVariant {
                            image: texture,
                            src_rect: source_rect,
                            rect: rect
                        },
                    border_radius: (),
                    corner: ()
                } = self.shape;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque 
            // or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 
                || back_end.has_texture_alpha(texture);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                triangulation::rect_tri_list_xy_f32(self.transform, rect),
                triangulation::rect_tri_list_rgba_f32(color),
                triangulation::rect_tri_list_uv_f32(texture, source_rect)
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize>
Draw<B, I>
for EllipseBorderColorContext {
    #[inline(always)]
    fn draw( &self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let color = self.color;
            let shape::Shape {
                    variant: shape::EllipseVariant(rect),
                    border_radius: border_radius,
                    corner: (),
                } = self.shape;
            // Complte transparency does  not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not complete opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_ellipse_border_tri_list_xy_f32_rgba_f32(
                128,
                self.transform,
                rect,
                color,
                border_radius,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        }
    }
}


impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for BevelRectangleColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let shape::RectangleVariant(rect) = self.shape.variant;
            let shape::BevelCorner(bevel_radius) = self.shape.corner;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_round_rectangle_tri_list_xy_f32_rgba_f32(
                2,
                self.transform,
                rect,
                bevel_radius,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for BevelRectangleBorderColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let shape::RectangleVariant(rect) = self.shape.variant;
            let shape::BevelCorner(bevel_radius) = self.shape.corner;
            let border_radius = self.shape.border_radius;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_round_rectangle_border_tri_list_xy_f32_rgba_f32(
                2,
                self.transform,
                rect,
                bevel_radius,
                border_radius,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for BevelBorderLineColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let shape::LineVariant(line) = self.shape.variant;
            let shape::BevelCorner(bevel_border_radius) = self.shape.corner;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_round_border_line_tri_list_xy_f32_rgba_f32(
                3,
                self.transform,
                line,
                bevel_border_radius,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for RoundRectangleColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let shape::RectangleVariant(rect) = self.shape.variant;
            let shape::RoundCorner(round_radius) = self.shape.corner;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_round_rectangle_tri_list_xy_f32_rgba_f32(
                32,
                self.transform,
                rect,
                round_radius,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<B: BackEnd<I>, I: ImageSize>
Draw<B, I>
for RoundRectangleBorderColorContext {
    #[inline(always)]
    fn draw( &self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let shape::RectangleVariant(rect) = self.shape.variant;
            let color = self.color;
            let border_radius = self.shape.border_radius;
            let shape::RoundCorner(round_radius) = self.shape.corner;
            // Complete transparency does  not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not complete opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_round_rectangle_border_tri_list_xy_f32_rgba_f32(
                128,
                self.transform,
                rect,
                round_radius,
                border_radius,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        }
    }
}


impl<B: BackEnd<I>, I: ImageSize> 
Draw<B, I> 
for SquareBorderLineColorContext {
    #[inline(always)]
    fn draw(&self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let shape::LineVariant(line) = self.shape.variant;
            let shape::SquareCorner(square_border_radius) = self.shape.corner;
            let color = self.color;
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            triangulation::with_round_border_line_tri_list_xy_f32_rgba_f32(
                2,
                self.transform,
                line,
                square_border_radius,
                color,
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

