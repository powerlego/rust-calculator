//! Implementation of the calculation buffer.

use gtk::subclass::prelude::*;
use gtk::{glib, EntryBuffer};

/// The calculation buffer. It is a subclass of [`gtk::EntryBuffer`] and is used to store the calculation that the user
/// is currently inputting.
#[derive(Debug, Default)]
pub struct CalculationBuffer {}

#[glib::object_subclass]
impl ObjectSubclass for CalculationBuffer {
    type ParentType = EntryBuffer;
    type Type = super::CalculationBuffer;

    const NAME: &'static str = "CalculationBuffer";
}

impl ObjectImpl for CalculationBuffer {}

impl EntryBufferImpl for CalculationBuffer {}
