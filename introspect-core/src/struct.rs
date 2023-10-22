//! Rust structs.

mod builder;
pub mod field;

pub use builder::Builder;
pub use field::Field;

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

/// A struct.
#[derive(Debug)]
pub struct Struct {
    identifier: String,

    documentation: Option<String>,
}

impl Struct {
    /// Creates a new [`Struct`].
    ///
    /// # Examples
    ///
    /// ```
    /// use introspect_core as core;
    ///
    /// let struct_ = core::Struct::new(
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

    /// Gets the identifier of the [`Struct`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use introspect_core as core;
    ///
    /// let struct_ = core::r#struct::Builder::default()
    ///                 .identifier("Name")
    ///                 .documentation("Documentation.")
    ///                 .try_build()?;
    ///
    /// assert_eq!(struct_.identifier(), "Name");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn identifier(&self) -> &str {
        self.identifier.as_str()
    }

    /// Gets the documentation of the [`Struct`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use introspect_core as core;
    ///
    /// let struct_ = core::r#struct::Builder::default()
    ///                 .identifier("Name")
    ///                 .documentation("Documentation.")
    ///                 .try_build()?;
    ///
    /// assert_eq!(struct_.documentation(), Some("Documentation."));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn documentation(&self) -> Option<&str> {
        self.documentation.as_deref()
    }
}

impl std::fmt::Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "::introspect::Struct::new(")?;
        write!(f, "\"{}\".into(), ", self.identifier)?;

        match self.documentation.as_ref() {
            Some(documentation) => write!(f, "Some(\"{}\".into())", documentation)?,
            None => write!(f, "None")?,
        };

        write!(f, ")")
    }
}

impl TryFrom<&syn::ItemStruct> for Struct {
    type Error = Error;

    fn try_from(value: &syn::ItemStruct) -> Result<Self> {
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
            identifier: value.ident.to_string(),
            documentation: match documentation.is_empty() {
                true => None,
                false => Some(documentation),
            },
        })
    }
}
