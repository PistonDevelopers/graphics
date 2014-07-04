use {
    AddBevel,
    AddBorder,
    AddRound,
    AddPolygons,
};
use internal::{
    CanRectangle,
    CanSourceRectangle,
    HasRectangle,
    HasSourceRectangle,
    Line,
    Polygons,
    Radius,
    Rectangle,
    Scalar,
    SourceRectangle,
};

pub struct RectangleVariant(pub Rectangle);
pub struct EllipseVariant(pub Rectangle);
pub struct LineVariant(pub Line);
pub struct ImageVariant<'a, I, TRectangle> {
    pub image: &'a I, 
    pub src_rect: SourceRectangle,
    pub rect: TRectangle,
}
pub struct LerpTweenVariant<TShapes> {
    pub lerp: Scalar,
    pub shapes: TShapes,
}

pub struct BevelCorner(pub Radius);
pub struct RoundCorner(pub Radius);
pub struct SquareCorner(pub Radius);

pub type EllipseShape = Shape<EllipseVariant, (), ()>;
pub type EllipseBorderShape = Shape<EllipseVariant, Radius, ()>;
pub type BevelRectangleShape = Shape<RectangleVariant, (), BevelCorner>;
pub type BevelRectangleBorderShape = Shape<RectangleVariant, Radius, BevelCorner>;
pub type ImageShape<'a, I> = Shape<ImageVariant<'a, I, ()>, (), ()>;
pub type ImageRectangleShape<'a, I> = Shape<ImageVariant<'a, I, Rectangle>, (), ()>;
pub type LerpTweenShape = Shape<LerpTweenVariant<()>, (), ()>;
pub type LerpTweenPolygonsShape<'a> = Shape<LerpTweenVariant<Polygons<'a>>, (), ()>;
pub type LineShape = Shape<LineVariant, (), ()>;
pub type BevelBorderLineShape = Shape<LineVariant, (), BevelCorner>;
pub type RoundBorderLineShape = Shape<LineVariant, (), RoundCorner>;
pub type SquareBorderLineShape = Shape<LineVariant, (), SquareCorner>;
pub type RectangleShape = Shape<RectangleVariant, (), ()>;
pub type RectangleBorderShape = Shape<RectangleVariant, Radius, ()>;
pub type RoundRectangleShape = Shape<RectangleVariant, (), RoundCorner>;
pub type RoundRectangleBorderShape = Shape<RectangleVariant, Radius, RoundCorner>;


/// A rectangle shape.
#[deriving(Copy)]
pub struct Shape<
    TVariant, 
    TBorderRadius,
    TCorner
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
for Shape<S, B, ()> {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: Radius
    ) -> Shape<S, B, BevelCorner> {
        Shape {
            variant: self.variant,
            corner: BevelCorner(radius),
            border_radius: self.border_radius,
        }
    }
}

impl<S: Copy, B: Copy>
AddRound<Shape<S, B, RoundCorner>> 
for Shape<S, B, ()> {
    #[inline(always)]
    fn round(
        &self, 
        radius: Radius
    ) -> Shape<S, B, RoundCorner> {
        Shape {
            variant: self.variant,
            corner: RoundCorner(radius),
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
impl<B: Copy, C: Copy>
CanRectangle<Shape<RectangleVariant, B, C>, Rectangle> 
for Shape<RectangleVariant, B, C> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<RectangleVariant, B, C> {
        Shape { 
            variant: RectangleVariant(rect), 
            ..*self 
        }
    }
}

/// Shapes containing an ellipse can change rectangle.
///
/// Can not contain a rectangle.
impl<B: Copy, C: Copy>
CanRectangle<Shape<EllipseVariant, B, C>, Rectangle> 
for Shape<EllipseVariant, B, C> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<EllipseVariant, B, C> {
        Shape { 
            variant: EllipseVariant(rect), 
            ..*self 
        }
    }
}


/// Gets rectangle of rectangle shape.
impl<B, C>
HasRectangle<Rectangle>
for Shape<RectangleVariant, B, C> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        let RectangleVariant(res) = self.variant;
        res
    }
}

/// Gets ellipse of ellipse shape.
impl<B, C>
HasRectangle<Rectangle>
for Shape<EllipseVariant, B, C> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        let EllipseVariant(res) = self.variant;
        res
    }
}

impl<'b, I> 
HasSourceRectangle<SourceRectangle> 
for ImageRectangleShape<'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&self) -> SourceRectangle {
        self.variant.src_rect
    }
}

impl<'b, I> 
CanSourceRectangle<ImageRectangleShape<'b, I>, SourceRectangle> 
for ImageRectangleShape<'b, I> {
    #[inline(always)]
    fn source_rectangle(
        &self, 
        source_rect: SourceRectangle
    ) -> ImageRectangleShape<'b, I> {
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

