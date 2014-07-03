
use {
    AddBevel,
    AddBorder,
    AddColor,
    AddEllipse,
    AddImage,
    AddLine,
    AddPolygon,
    AddRectangle,
    AddRound,
    AddTween,
    BackEnd,
    BevelRectangleContext,
    BevelRectangleBorderContext,
    ColorContext,
    Draw,
    EllipseContext,
    EllipseBorderContext,
    ImageSize,
    ImageContext,
    ImageRectangleContext,
    LineContext,
    LerpTweenContext,
    PolygonColorContext,
    RectangleContext,
    RectangleColorContext,
    RectangleBorderContext,
    RectangleBorderColorContext,
    RoundRectangleContext,
    RoundRectangleBorderContext,
};
use triangulation::{
    with_polygon_tri_list_xy_f32_rgba_f32,
};
use internal::{
    CanColor,
    CanRectangle,
    CanTransform,
    CanViewTransform,
    Color,
    ColorComponent,
    HasColor,
    HasRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Radius,
    Rectangle,
    Polygon,
    Scalar,
};
use shape;

/// Drawing 2d context.
#[deriving(Copy)]
pub struct Context<S, C> {
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

    /// Creates a new drawing context.
    #[inline(always)]
    pub fn ctx_id() -> Context<(), ()> {
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
    pub fn ctx_abs(w: f64, h: f64) -> Context<(), ()> {
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

#[test]
fn test_context() {
    use RelativeTransform2d;

    let c = ctx_id();
    {
        let d = c.trans(20.0, 40.0);
        let d = d.trans(10.0, 10.0);
        let transform = d.transform.get();
        assert_eq!(transform[2], 30.0);
        assert_eq!(transform[5], 50.0);
    }
    
    let transform = c.transform.get();
    assert_eq!(transform[2], 0.0);
    assert_eq!(transform[5], 0.0);

    let c = c.rot_deg(90.0);
    let transform = c.transform.get();
    assert!((transform[0] - 0.0).abs() < 0.00001);
    assert!((transform[1] + 1.0).abs() < 0.00001);
}

#[test]
fn test_scale() {
    use RelativeTransform2d;

    let c = ctx_id();
    let c = c.scale(2.0, 3.0);
    let transform = c.transform.get();
    assert!((transform[0] - 2.0).abs() < 0.00001);
    assert!((transform[4] - 3.0).abs() < 0.00001);
}

impl<C: Copy>
AddRectangle<Context<shape::RectangleShape, C>> 
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
                round_radius: (),
                bevel_radius: (), 
            },
        }
    }
}

#[test]
fn test_rect() {
    let c = ctx_id();
    let d = c.rect(0.0, 0.0, 100.0, 50.0);
    let rect = d.rect.get();
    assert_eq!(rect[2], 100.0);
}

impl<S: Copy>
AddColor<Context<S, Color>> 
for Context<S, ()> {
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
    let c = ctx_id();
    let d: ColorContext = c.rgba(1.0, 0.0, 0.0, 1.0);
    let color = d.color;
    assert_eq!(color[0], 1.0);
}

impl<C: Copy>
AddEllipse<Context<shape::EllipseShape, C>> 
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
                round_radius: (),
                bevel_radius: (), 
            },
        }
    }
}

#[test]
fn test_ellipse() {
    let c = ctx_id();
    let d: EllipseContext = c.ellipse(0.0, 0.0, 100.0, 100.0);
    let rect = d.rect;    
    assert_eq!(rect[2], 100.0);
}

impl<'b, C: Copy> 
AddPolygon<'b, Context<Polygon<'b>, C>> 
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

impl<'b, I: ImageSize> 
AddImage<'b, ImageContext<'b, I>, I> 
for Context<(), ()> {
    #[inline(always)]
    fn image(&self, image: &'b I) -> ImageContext<'b, I> {
        let (w, h) = image.get_size();
        ImageContext {
            view: self.view,
            transform: self.transform,
            image: image,
            source_rect: [0, 0, w as i32, h as i32],
        }
    }
}

impl
AddTween<LerpTweenContext> 
for Context<(), ()> {
    #[inline(always)]
    fn lerp(&self, tween_factor: Scalar) -> LerpTweenContext {
        LerpTweenContext {
            view: self.view,
            transform: self.transform,
            tween_factor: tween_factor,
        }
    }
}

impl
AddLine<LineContext> 
for Context<(), ()> {
    #[inline(always)]
    fn line(
        &self, 
        x1: Scalar, 
        y1: Scalar, 
        x2: Scalar, 
        y2: Scalar
    ) -> LineContext {
        LineContext {
            view: self.view,
            transform: self.transform,
            line: [x1, y1, x2, y2],
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

impl<'b, I: ImageSize> 
AddImage<'b, ImageRectangleContext<'b, I>, I> 
for RectangleContext {
    fn image(
        &self, 
        image: &'b I
    ) -> ImageRectangleContext<'b, I> {
        let (w, h) = image.get_size();
        ImageRectangleContext {
            view: self.view,
            transform: self.transform,
            rect: self.shape.get_rectangle(),
            image: image,
            source_rect: [0, 0, w as i32, h as i32],
        }
    }
}

impl<S: AddRound<S2>, S2: Copy, C: Copy>
AddRound<Context<S2, C>> 
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
            with_polygon_tri_list_xy_f32_rgba_f32(
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
    let c = ctx_id();
    let d = c.rect(0.0, 0.0, 100.0, 100.0);
    let e = d.rgba(1.0, 0.0, 0.0, 1.0);
    let color = e.color;
    assert_eq!(color[0], 1.0);
}

