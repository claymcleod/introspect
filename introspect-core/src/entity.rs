use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::Enum;
use crate::Struct;

/// A member of a Rust construct.
#[derive(Debug)]
pub enum Entity {
    /// An enum.
    Enum(Enum),

    /// A struct.
    Struct(Struct),
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Struct(struct_) => write!(f, "::introspect::Entity::Struct({})", struct_),
            Entity::Enum(enum_) => write!(f, "::introspect::Entity::Enum({})", enum_),
        }
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // SAFETY: this unwrap should never fail as we exhaustively test converting a
        // [`Entity`] to a string that eventually parses to a token stream.
        tokens.extend(self.to_string().parse::<TokenStream>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_enum_converts_to_string_correctly() {
        let member = Entity::Enum(Enum::new("Name".into(), Some("Documentation.".into())));

        assert_eq!(member.to_string(), "::introspect::Entity::Enum(::introspect::Enum::new(\"Name\".into(), Some(\"Documentation.\".into())))");
    }

    #[test]
    fn a_struct_converts_to_string_correctly() {
        let member = Entity::Struct(Struct::new("Name".into(), Some("Documentation.".into())));

        assert_eq!(member.to_string(), "::introspect::Entity::Struct(::introspect::Struct::new(\"Name\".into(), Some(\"Documentation.\".into())))");
    }
}
