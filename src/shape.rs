use {
    AddBevel,
    AddBorder,
    AddRound,
};
use internal::{
    CanRectangle,
    HasRectangle,
    Radius,
    Rectangle,
};

pub struct RectangleVariant(pub Rectangle);
pub struct EllipseVariant(pub Rectangle);

pub type EllipseShape = Shape<EllipseVariant, (), (), ()>;
pub type EllipseBorderShape = Shape<EllipseVariant, Radius, (), ()>;
pub type BevelRectangleShape = Shape<RectangleVariant, (), (), Radius>;
pub type BevelRectangleBorderShape = Shape<RectangleVariant, Radius, (), Radius>;
pub type RectangleShape = Shape<RectangleVariant, (), (), ()>;
pub type RectangleBorderShape = Shape<RectangleVariant, Radius, (), ()>;
pub type RoundRectangleShape = Shape<RectangleVariant, (), Radius, ()>;
pub type RoundRectangleBorderShape = Shape<RectangleVariant, Radius, Radius, ()>;

/// A rectangle shape.
#[deriving(Copy)]
pub struct Shape<
    TVariant, 
    TBorderRadius,
    TRoundRadius,
    TBevelRadius
> {
    /// Rectangle.
    pub variant: TVariant,
    /// Border radius.
    pub border_radius: TBorderRadius,
    /// Round radius.
    pub round_radius: TRoundRadius,
    /// Bevel radius.
    pub bevel_radius: TBevelRadius,
}


impl
AddRound<RoundRectangleShape> 
for RectangleShape {
    #[inline(always)]
    fn round(
        &self, 
        radius: Radius
    ) -> RoundRectangleShape {
        Shape {
            variant: self.variant,
            round_radius: radius,
            border_radius: self.border_radius,
            bevel_radius: self.bevel_radius,
        }
    }
}

impl
AddRound<RoundRectangleBorderShape> 
for RectangleBorderShape {
    #[inline(always)]
    fn round(&self, radius: Radius) -> RoundRectangleBorderShape {
        Shape {
            variant: self.variant,
            border_radius: self.border_radius,
            round_radius: radius,
            bevel_radius: self.bevel_radius,
        }
    }
}

impl
AddBevel<BevelRectangleShape> 
for RectangleShape {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: Radius
    ) -> BevelRectangleShape {
        Shape {
            variant: self.variant,
            bevel_radius: radius,
            round_radius: self.round_radius,
            border_radius: self.border_radius,
        }
    }
}


impl
AddBorder<EllipseBorderShape> 
for EllipseShape {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> EllipseBorderShape {
        Shape {
            border_radius: radius,
            variant: self.variant,
            round_radius: self.round_radius,
            bevel_radius: self.bevel_radius,
        }
    }
}

impl
AddBorder<RectangleBorderShape> 
for RectangleShape {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> RectangleBorderShape {
        Shape {
            border_radius: radius,
            variant: self.variant,
            round_radius: self.round_radius,
            bevel_radius: self.bevel_radius,
        }
    }
}

impl
AddBevel<BevelRectangleBorderShape> 
for RectangleBorderShape {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: Radius
    ) -> BevelRectangleBorderShape {
        Shape {
            bevel_radius: radius,
            variant: self.variant,
            border_radius: self.border_radius,     
            round_radius: self.round_radius,
        }
    }
}

/// Shapes containing a rectangle can change rectangle.
///
/// Can not contain an ellipse.
impl<B: Copy, C: Copy, D: Copy>
CanRectangle<Shape<RectangleVariant, B, C, D>, Rectangle> 
for Shape<RectangleVariant, B, C, D> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<RectangleVariant, B, C, D> {
        Shape { 
            variant: RectangleVariant(rect), 
            ..*self 
        }
    }
}

/// Shapes containing an ellipse can change rectangle.
///
/// Can not contain a rectangle.
impl<B: Copy, C: Copy, D: Copy>
CanRectangle<Shape<EllipseVariant, B, C, D>, Rectangle> 
for Shape<EllipseVariant, B, C, D> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<EllipseVariant, B, C, D> {
        Shape { 
            variant: EllipseVariant(rect), 
            ..*self 
        }
    }
}


/// Gets rectangle of rectangle shape.
impl<B, C, D>
HasRectangle<Rectangle>
for Shape<RectangleVariant, B, C, D> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        let RectangleVariant(res) = self.variant;
        res
    }
}

/// Gets ellipse of ellipse shape.
impl<B, C, D>
HasRectangle<Rectangle>
for Shape<EllipseVariant, B, C, D> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        let EllipseVariant(res) = self.variant;
        res
    }
}

