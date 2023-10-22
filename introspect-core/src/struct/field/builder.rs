use crate::r#struct::Field;

/// A builder for a [`Field`].
#[derive(Debug, Default)]
pub struct Builder {
    /// An identifier for the field, if it exists.
    identifier: Option<String>,

    /// The documentation for the field, if it exists.
    documentation: Option<String>,
}

impl Builder {
    /// Sets the identifier for this [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use introspect_core as core;
    ///
    /// let builder = core::r#struct::field::Builder::default()
    ///                 .identifier("Name");
    /// ```
    pub fn identifier<S: Into<String>>(mut self, value: S) -> Self {
        self.identifier = Some(value.into());
        self
    }

    /// Sets the documentation for this [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use introspect_core as core;
    ///
    /// let builder = core::r#struct::field::Builder::default()
    ///                 .documentation("Documentation.");
    /// ```
    pub fn documentation<S: Into<String>>(mut self, value: S) -> Self {
        self.documentation = Some(value.into());
        self
    }

    /// Consume `self` to produce an immutable [`Field`].
    ///
    /// # Examples
    ///
    /// ```
    /// use introspect_core as core;
    ///
    /// let field = core::r#struct::field::Builder::default()
    ///                 .identifier("Name")
    ///                 .documentation("Documentation.")
    ///                 .build();
    /// ```
    pub fn build(self) -> Field {
        Field {
            identifier: self.identifier,
            documentation: self.documentation,
        }
    }
}
