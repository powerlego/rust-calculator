//! Implementation of the calculation buffer.

use gtk::{glib,EntryBuffer};
use gtk::subclass::prelude::*;

/// The calculation buffer. It is a subclass of [`gtk::EntryBuffer`] and is used to store the calculation that the user is currently inputting.

#[derive(Debug, Default)]
pub struct CalculationBuffer {
}

#[glib::object_subclass]
impl ObjectSubclass for CalculationBuffer {
    const NAME: &'static str = "CalculationBuffer";
    type Type = super::CalculationBuffer;
    type ParentType = EntryBuffer;
}

impl ObjectImpl for CalculationBuffer {}

impl EntryBufferImpl for CalculationBuffer {}