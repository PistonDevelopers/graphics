use {
    AddBevel,
    AddBorder,
    AddRound,
    AddPolygons,
};
use internal::{
    Color,
    Line,
    Polygon,
    Polygons,
    Radius,
    Rectangle,
    SourceRectangle,
};
use can::{
    CanRectangle,
    CanSourceRectangle,
};
use has::{
    HasColor,
    HasRectangle,
    HasSourceRectangle,
};
use vecmath::Scalar;

pub struct RectangleVariant {
    pub rect: Rectangle
}
pub struct EllipseVariant {
    pub rect: Rectangle
}
pub struct LineVariant {
    pub line: Line
}
pub struct ImageVariant<'a, I:'a, TRectangle=()> {
    pub image: &'a I,
    pub src_rect: SourceRectangle,
    pub rect: TRectangle,
}
pub struct LerpTweenVariant<TShapes=()> {
    pub lerp: Scalar,
    pub shapes: TShapes,
}
pub struct PolygonVariant<'a> {
    pub polygon: Polygon<'a>
}

pub struct BevelCorner {
    pub bevel_radius: Radius
}
pub struct RoundCorner {
    pub round_radius: Radius
}
pub struct SquareCorner {
    pub square_radius: Radius
}

pub type EllipseShape = Shape<EllipseVariant>;
pub type EllipseBorderShape = Shape<EllipseVariant, Radius>;
pub type BevelRectangleShape = Shape<RectangleVariant, (), BevelCorner>;
pub type BevelRectangleBorderShape = Shape<RectangleVariant, Radius, BevelCorner>;
pub type ImageShape<'a, I> = Shape<ImageVariant<'a, I>>;
pub type ImageRectangleShape<'a, I> = Shape<ImageVariant<'a, I, Rectangle>>;
pub type LerpTweenShape = Shape<LerpTweenVariant>;
pub type LerpTweenPolygonsShape<'a> = Shape<LerpTweenVariant<Polygons<'a>>>;
pub type LineShape = Shape<LineVariant>;
pub type BevelBorderLineShape = Shape<LineVariant, (), BevelCorner>;
pub type RoundBorderLineShape = Shape<LineVariant, (), RoundCorner>;
pub type SquareBorderLineShape = Shape<LineVariant, (), SquareCorner>;
pub type PolygonShape<'a> = Shape<PolygonVariant<'a>, (), ()>;
pub type RectangleShape = Shape<RectangleVariant>;
pub type RectangleBorderShape = Shape<RectangleVariant, Radius>;
pub type RoundRectangleShape = Shape<RectangleVariant, (), RoundCorner>;
pub type RoundRectangleBorderShape = Shape<RectangleVariant, Radius, RoundCorner>;


/// A rectangle shape.
#[deriving(Copy)]
pub struct Shape<
    TVariant,
    TBorderRadius=(),
    TCorner=()
> {
    /// Rectangle.
    pub variant: TVariant,
    /// Border radius.
    pub border_radius: TBorderRadius,
    /// Round radius.
    pub corner: TCorner,
}

impl<S: Copy, B: Copy>
AddBevel<Shape<S, B, BevelCorner>>
for Shape<S, B> {
    #[inline(always)]
    fn bevel(
        &self,
        radius: Radius
    ) -> Shape<S, B, BevelCorner> {
        Shape {
            variant: self.variant,
            corner: BevelCorner { bevel_radius: radius },
            border_radius: self.border_radius,
        }
    }
}

impl<S: Copy, B: Copy>
AddRound<Shape<S, B, RoundCorner>>
for Shape<S, B> {
    #[inline(always)]
    fn round(
        &self,
        radius: Radius
    ) -> Shape<S, B, RoundCorner> {
        Shape {
            variant: self.variant,
            corner: RoundCorner { round_radius: radius },
            border_radius: self.border_radius,
        }
    }
}

impl<S: Copy, C: Copy>
AddBorder<Shape<S, Radius, C>>
for Shape<S, (), C> {
    #[inline(always)]
    fn border_radius(
        &self,
        radius: Radius
    ) -> Shape<S, Radius, C> {
        Shape {
            variant: self.variant,
            corner: self.corner,
            border_radius: radius,
        }
    }
}

/// Shapes containing a rectangle can change rectangle.
///
/// Can not contain an ellipse.
impl<B: Copy, C: Copy> CanRectangle for Shape<RectangleVariant, B, C> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<RectangleVariant, B, C> {
        Shape {
            variant: RectangleVariant { rect: rect },
            ..*self
        }
    }
}

/// Shapes containing an ellipse can change rectangle.
///
/// Can not contain a rectangle.
impl<B: Copy, C: Copy> CanRectangle for Shape<EllipseVariant, B, C> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<EllipseVariant, B, C> {
        Shape {
            variant: EllipseVariant { rect: rect },
            ..*self
        }
    }
}

impl<'b, I, B: Copy, C: Copy>
CanRectangle
for Shape<ImageVariant<'b, I, Rectangle>, B, C> {
    #[inline(always)]
    fn rectangle(
        &self,
        rect: Rectangle
    ) -> Shape<ImageVariant<'b, I, Rectangle>, B, C> {
        Shape {
            variant: ImageVariant {
                    rect: rect,
                    image: self.variant.image,
                    src_rect: self.variant.src_rect
                },
            border_radius: self.border_radius,
            corner: self.corner,
        }
    }
}


/// Gets rectangle of rectangle shape.
impl<B, C>
HasRectangle<Rectangle>
for Shape<RectangleVariant, B, C> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.variant.rect
    }
}

/// Gets ellipse of ellipse shape.
impl<B, C>
HasRectangle<Rectangle>
for Shape<EllipseVariant, B, C> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.variant.rect
    }
}

impl<'b, I, B, C>
HasRectangle<Rectangle>
for Shape<ImageVariant<'b, I, Rectangle>, B, C> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.variant.rect
    }
}


impl<'b, I, R, B, C>
HasSourceRectangle<SourceRectangle>
for Shape<ImageVariant<'b, I, R>, B, C> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> SourceRectangle {
        self.variant.src_rect
    }
}

impl<'b, I, R: Copy, B: Copy, C: Copy>
CanSourceRectangle
for Shape<ImageVariant<'b, I, R>, B, C> {
    #[inline(always)]
    fn source_rectangle(
        &self,
        source_rect: SourceRectangle
    ) -> Shape<ImageVariant<'b, I, R>, B, C> {
        Shape {
            variant: ImageVariant {
                    image: self.variant.image,
                    rect: self.variant.rect,
                    src_rect: source_rect,
                },
            border_radius: self.border_radius,
            corner: self.corner,
        }
    }
}

impl<'b>
AddPolygons<'b, LerpTweenPolygonsShape<'b>>
for LerpTweenShape {
    #[inline(always)]
    fn polygons(
        &self,
        polygons: Polygons<'b>
    ) -> LerpTweenPolygonsShape<'b> {
        Shape {
            variant: LerpTweenVariant {
                    lerp: self.variant.lerp,
                    shapes: polygons,
                },
            border_radius: self.border_radius,
            corner: self.corner,
        }
    }
}

static WHITE: Color = [1.0, ..4];

// Use white color per vertex.
impl<'b, I, R, B, C>
HasColor<Color>
for Shape<ImageVariant<'b, I, R>, B, C> {
    #[inline(always)]
    fn get_color(&self) -> Color {
        WHITE
    }
}
