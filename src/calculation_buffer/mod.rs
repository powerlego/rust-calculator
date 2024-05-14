mod imp;

use glib::Object;
use glib::prelude::*;
use gtk::glib;

glib::wrapper! {
    pub struct CalculationBuffer(ObjectSubclass<imp::CalculationBuffer>);
}

impl CalculationBuffer {
    pub fn new() -> Self {
        Object::builder().property("text", "").build()
    }

    /// Creates a new builder-pattern struct instance to construct [`CalculationBuffer`] objects.
    ///
    /// This method returns an instance of [`CalculationBufferBuilder`] which can be used to create [`CalculationBuffer`] objects.
    pub fn builder() -> CalculationBufferBuilder {
        CalculationBufferBuilder::new()
    }
}


/// A [builder-pattern] type to construct [`CalculationBuffer`] objects.
///
#[must_use ="The builder must be built to be used."]
pub struct CalculationBufferBuilder {
    builder: glib::object::ObjectBuilder<'static,CalculationBuffer>
}

impl CalculationBufferBuilder {
    /// Create a new [`CalculationBufferBuilder`]
    pub fn new() -> Self {
        Self {
            builder: glib::object::Object::builder()
        }
    }

    /// Set the text of the [`CalculationBuffer`]
    pub fn text(self, text: &str) -> Self {
        Self {
            builder: self.builder.property("text", text)
        }
    }

    /// Build the [`CalculationBuffer`]
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CalculationBuffer {
        self.builder.build()
    }
}