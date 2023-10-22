use crate::Enum;

/// An error related to a [`Builder`].
#[derive(Debug)]
pub enum Error {
    /// An identifier was never added to the [`Builder`].
    MissingIdentifier,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingIdentifier => write!(f, "missing identifier"),
        }
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// A builder for a [`Enum`].
#[derive(Debug, Default)]
pub struct Builder {
    /// An identifier for the enum.
    identifier: Option<String>,

    /// The documentation for the enum, if it exists.
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
    /// let builder = core::r#enum::Builder::default()
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
    /// let builder = core::r#enum::Builder::default()
    ///                 .documentation("Documentation.");
    /// ```
    pub fn documentation<S: Into<String>>(mut self, value: S) -> Self {
        self.documentation = Some(value.into());
        self
    }

    /// Consume `self` to produce an immutable [`Enum`].
    ///
    /// # Examples
    ///
    /// ```
    /// use introspect_core as core;
    ///
    /// let enum_ = core::r#enum::Builder::default()
    ///                 .identifier("Name")
    ///                 .documentation("Documentation.")
    ///                 .try_build()?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_build(self) -> Result<Enum> {
        let identifier = match self.identifier {
            Some(identifier) => identifier,
            None => return Err(Error::MissingIdentifier),
        };

        Ok(Enum {
            identifier,
            documentation: self.documentation,
        })
    }
}
