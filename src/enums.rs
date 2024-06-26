use glib::prelude::*;
use gtk::glib;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, glib::Enum)]
#[enum_type(name = "Modifier")]
pub enum Modifier {
    Negate,
    Percent,
    OneOver,
    Square,
    SquareRoot,
    Cube,
    CubeRoot,
    Abs,
    Floor,
    Ceil,
    Exponent,
    Factorial,
    TenToTheX,
    TwoToTheX,
    Ln,
    Log,
    EToTheX,
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    ArcSin,
    ArcCos,
    ArcTan,
    ArcCsc,
    ArcSec,
    ArcCot,
    Sinh,
    Cosh,
    Tanh,
    Csch,
    Sech,
    Coth,
    ArcSinh,
    ArcCosh,
    ArcTanh,
    ArcCsch,
    ArcSech,
    ArcCoth,
    ToDegrees,
    ToDMS,
    None,
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier::None
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, glib::Enum)]
#[enum_type(name = "Operator")]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
    NthRoot,
    LogBaseY,
    And,
    Or,
    Xor,
    Not,
    Nor,
    Nand,
    None
}

impl Default for Operator {
    fn default() -> Self {
        Operator::None
    }
}