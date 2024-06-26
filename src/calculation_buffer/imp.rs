//! Implementation of the calculation buffer.
use std::cell::RefCell;

use glib::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, EntryBuffer};

use crate::enums::{Modifier, Operator};

#[derive(Default, Clone)]
pub struct Operand {
    pub value:     f64,
    pub modifiers:  Vec<Modifier>,
}

/// The calculation buffer. It is a subclass of [`gtk::EntryBuffer`] and is used to store the calculation that the user
/// is currently inputting.
#[derive(Default)]
pub struct CalculationBuffer {
    left_operand:  RefCell<Operand>,
    right_operand: RefCell<Operand>,
    operator:      RefCell<Operator>,
}

#[glib::object_subclass]
impl ObjectSubclass for CalculationBuffer {
    type ParentType = EntryBuffer;
    type Type = super::CalculationBuffer;

    const NAME: &'static str = "CalculationBuffer";
}

impl ObjectImpl for CalculationBuffer {}

impl EntryBufferImpl for CalculationBuffer {}
