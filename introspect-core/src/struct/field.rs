//! Rust struct fields.

mod builder;

pub use builder::Builder;

/// An error related to a [`Field`].
pub enum Error {
    /// Encountered an unsupported expression for a documentation attribute.
    UnsupportedExpression(syn::Expr),

    /// Encountered an unsupported expression literal for a documentation attribute.
    UnsupportedExpressionLiteral(syn::ExprLit),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedExpression(_) => f.debug_tuple("UnsupportedExpression").finish(),
            Self::UnsupportedExpressionLiteral(_) => {
                f.debug_tuple("UnsupportedExpressionLiteral").finish()
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedExpression(_) => {
                write!(f, "unsupported doc attribute expression")
            }
            Error::UnsupportedExpressionLiteral(_) => {
                write!(f, "unsupported doc attribute literal")
            }
        }
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// A Rust struct field.
#[derive(Debug)]
pub struct Field {
    /// An identifier for the field, if it exists.
    identifier: Option<String>,

    /// The documentation for the field, if it exists.
    documentation: Option<String>,
}

impl Field {
    /// Creates a new [`Field`].
    ///
    /// # Examples
    ///
    /// ```
    /// use introspect_core as core;
    ///
    /// let field = core::r#struct::Field::new(
    ///     Some(String::from("Name")),
    ///     Some(String::from("Documentation."))
    /// );
    /// ```
    pub fn new(identifier: Option<String>, documentation: Option<String>) -> Self {
        Self {
            identifier,
            documentation,
        }
    }

    /// Gets the identifier of the [`Field`] by reference.
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
    ///
    /// assert_eq!(field.identifier(), Some("Name"));
    /// ```
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }

    /// Gets the documentation of the [`Field`] by reference.
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
    ///
    /// assert_eq!(field.documentation(), Some("Documentation."));
    /// ```
    pub fn documentation(&self) -> Option<&str> {
        self.documentation.as_deref()
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "::introspect::r#struct::Field::new(")?;

        match self.identifier.as_ref() {
            Some(identifier) => write!(f, "Some(\"{}\".into())", identifier)?,
            None => write!(f, "None")?,
        };

        write!(f, ", ")?;

        match self.documentation.as_ref() {
            Some(documentation) => write!(f, "Some(\"{}\".into())", documentation)?,
            None => write!(f, "None")?,
        };

        write!(f, ")")
    }
}

impl TryFrom<&syn::Field> for Field {
    type Error = Error;

    fn try_from(value: &syn::Field) -> Result<Self> {
        let documentation = value
            .attrs
            .iter()
            .filter_map(|attr| match attr.meta.require_name_value() {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .filter_map(|field| {
                field
                    .path
                    .get_ident()
                    .map(|ident| (ident, field.value.clone()))
            })
            .filter(|(ident, _)| *ident == "doc")
            .map(|(_, expr)| match expr {
                syn::Expr::Lit(expr_lit) => match expr_lit.lit {
                    syn::Lit::Str(lit_str) => Ok(lit_str.value().trim().to_string()),
                    _ => Err(Error::UnsupportedExpressionLiteral(expr_lit)),
                },
                _ => Err(Error::UnsupportedExpression(expr)),
            })
            .collect::<Result<Vec<String>>>()?
            .join("\n");

        Ok(Self {
            identifier: value.ident.as_ref().map(|ident| ident.to_string()),
            documentation: match documentation.is_empty() {
                true => None,
                false => Some(documentation),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_identifier_and_documentation() {
        let field = Field::new(
            Some(String::from("Name")),
            Some(String::from("Documentation.")),
        );

        assert_eq!(
            field.to_string(),
            "::introspect::r#struct::Field::new(Some(\"Name\".into()), Some(\"Documentation.\".into()))"
        )
    }

    #[test]
    fn display_only_identifier() {
        let field = Field::new(Some(String::from("Name")), None);

        assert_eq!(
            field.to_string(),
            "::introspect::r#struct::Field::new(Some(\"Name\".into()), None)"
        )
    }

    #[test]
    fn display_only_documentation() {
        let field = Field::new(None, Some(String::from("Documentation.")));

        assert_eq!(
            field.to_string(),
            "::introspect::r#struct::Field::new(None, Some(\"Documentation.\".into()))"
        )
    }

    #[test]
    fn display_neither() {
        let field = Field::new(None, None);

        assert_eq!(
            field.to_string(),
            "::introspect::r#struct::Field::new(None, None)"
        )
    }
}
