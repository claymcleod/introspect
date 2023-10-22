//! Rust enums.

mod builder;
pub mod variant;

pub use builder::Builder;
pub use variant::Variant;

/// An error related to a [`Enum`].
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

/// An enum.
#[derive(Debug)]
pub struct Enum {
    identifier: String,

    documentation: Option<String>,
}

impl Enum {
    /// Creates a new [`Enum`].
    ///
    /// # Examples
    ///
    /// ```
    /// use introspect_core as core;
    ///
    /// let enum_ = core::Enum::new(
    ///     String::from("Name"),
    ///     Some(String::from("Documentation."))
    /// );
    /// ```
    pub fn new(identifier: String, documentation: Option<String>) -> Self {
        Self {
            identifier,
            documentation,
        }
    }

    /// Gets the identifier of the [`Enum`] by reference.
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
    /// assert_eq!(enum_.identifier(), "Name");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn identifier(&self) -> &str {
        self.identifier.as_str()
    }

    /// Gets the documentation of the [`Enum`] by reference.
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
    /// assert_eq!(enum_.documentation(), Some("Documentation."));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn documentation(&self) -> Option<&str> {
        self.documentation.as_deref()
    }
}

impl std::fmt::Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "::introspect::Enum::new(")?;
        write!(f, "\"{}\".into(), ", self.identifier)?;

        match self.documentation.as_ref() {
            Some(documentation) => write!(f, "Some(\"{}\".into())", documentation)?,
            None => write!(f, "None")?,
        };

        write!(f, ")")
    }
}

impl TryFrom<&syn::ItemEnum> for Enum {
    type Error = Error;

    fn try_from(value: &syn::ItemEnum) -> Result<Self> {
        let documentation = value
            .attrs
            .iter()
            .filter_map(|attr| match attr.meta.require_name_value() {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .filter_map(|enum_| {
                enum_
                    .path
                    .get_ident()
                    .map(|ident| (ident, enum_.value.clone()))
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
            identifier: value.ident.to_string(),
            documentation: match documentation.is_empty() {
                true => None,
                false => Some(documentation),
            },
        })
    }
}
