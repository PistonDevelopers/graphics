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

pub type EllipseShape = Shape<(), Rectangle, (), (), ()>;
pub type EllipseBorderShape = Shape<(), Rectangle, Radius, (), ()>;
pub type BevelRectangleShape = Shape<Rectangle, (), (), (), Radius>;
pub type BevelRectangleBorderShape = Shape<Rectangle, (), Radius, (), Radius>;
pub type RectangleShape = Shape<Rectangle, (), (), (), ()>;
pub type RectangleBorderShape = Shape<Rectangle, (), Radius, (), ()>;
pub type RoundRectangleShape = Shape<Rectangle, (), (), Radius, ()>;
pub type RoundRectangleBorderShape = Shape<Rectangle, (), Radius, Radius, ()>;

/// A rectangle shape.
#[deriving(Copy)]
pub struct Shape<
    TRectangle, 
    TEllipse, 
    TBorderRadius,
    TRoundRadius,
    TBevelRadius
> {
    /// Rectangle.
    pub rectangle: TRectangle,
    /// Ellipse.
    pub ellipse: TEllipse,
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
            rectangle: self.rectangle,
            round_radius: radius,
            ellipse: self.ellipse,
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
            rectangle: self.rectangle,
            border_radius: self.border_radius,
            round_radius: radius,
            ellipse: self.ellipse,
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
            rectangle: self.rectangle,
            bevel_radius: radius,
            ellipse: self.ellipse,
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
            ellipse: self.ellipse,
            border_radius: radius,
            rectangle: self.rectangle,
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
            ellipse: self.ellipse,
            border_radius: radius,
            rectangle: self.rectangle,
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
            rectangle: self.rectangle,
            ellipse: self.ellipse,
            border_radius: self.border_radius,     
            round_radius: self.round_radius,
        }
    }
}

/// Shapes containing a rectangle can change rectangle.
///
/// Can not contain an ellipse.
impl<B: Copy, C: Copy, D: Copy>
CanRectangle<Shape<Rectangle, (), B, C, D>, Rectangle> 
for Shape<Rectangle, (), B, C, D> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<Rectangle, (), B, C, D> {
        Shape { 
            rectangle: rect, 
            ..*self 
        }
    }
}

/// Shapes containing an ellipse can change rectangle.
///
/// Can not contain a rectangle.
impl<B: Copy, C: Copy, D: Copy>
CanRectangle<Shape<(), Rectangle, B, C, D>, Rectangle> 
for Shape<(), Rectangle, B, C, D> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<(), Rectangle, B, C, D> {
        Shape { 
            ellipse: rect, 
            ..*self 
        }
    }
}


/// Gets rectangle of rectangle shape.
impl<B, C, D>
HasRectangle<Rectangle>
for Shape<Rectangle, (), B, C, D> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rectangle
    }
}

/// Gets ellipse of ellipse shape.
impl<B, C, D>
HasRectangle<Rectangle>
for Shape<(), Rectangle, B, C, D> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.ellipse
    }
}

