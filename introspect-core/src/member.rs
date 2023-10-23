use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::r#enum::Variant;
use crate::r#struct::Field;

/// A member of a Rust construct.
#[derive(Debug)]
pub enum Member {
    /// A struct field.
    Field(Field),

    /// An enum variant.
    Variant(Variant),
}

impl std::fmt::Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Member::Field(field) => write!(f, "::introspect::Member::Field({})", field),
            Member::Variant(variant) => write!(f, "::introspect::Member::Variant({})", variant),
        }
    }
}

impl ToTokens for Member {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // SAFETY: this unwrap should never fail as we exhaustively test converting a
        // [`Member`] to a string that eventually parses to a token stream.
        tokens.extend(self.to_string().parse::<TokenStream>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_enum_variant_converts_to_string_correctly() {
        let member = Member::Variant(Variant::new("Name".into(), Some("Documentation.".into())));

        assert_eq!(member.to_string(), "::introspect::Member::Variant(::introspect::r#enum::Variant::new(r#\"Name\"#.into(), Some(r#\"Documentation.\"#.into())))");
    }

    #[test]
    fn a_struct_field_converts_to_string_correctly() {
        let member = Member::Field(Field::new(
            Some("Name".into()),
            Some("Documentation.".into()),
        ));

        assert_eq!(member.to_string(), "::introspect::Member::Field(::introspect::r#struct::Field::new(Some(r#\"Name\"#.into()), Some(r#\"Documentation.\"#.into())))");
    }
}
