use introspect::Entity;
use introspect::Introspect;
use introspect::IntrospectedEntity;
use introspect::IntrospectedMembers;
use introspect::Member;

/// This is the documentation for the [`ExampleOne`] struct.
#[allow(dead_code)]
#[derive(Debug, Introspect)]
struct ExampleOne {
    /// A foo.
    foo: String,

    /// A bar.
    ///
    /// This bar always confroms to baz.
    bar: usize,
}

#[derive(Introspect)]
struct ExampleTwo(usize);

pub fn main() {
    print_struct::<ExampleOne>();
    println!();

    print_struct::<ExampleTwo>();
}

fn print_struct<S: IntrospectedMembers + IntrospectedEntity>() {
    let struct_ = match S::introspected_entity() {
        Entity::Struct(struct_) => struct_,
        _ => unreachable!(),
    };

    println!("[{}]\n", struct_.identifier());

    if let Some(documentation) = struct_.documentation() {
        println!(
            "{}\n",
            documentation
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    for field in S::introspected_members() {
        match field {
            Member::Field(field) => {
                let ident = field
                    .identifier()
                    .map(|s| s.to_owned())
                    .unwrap_or(String::from("<unnamed>"));

                print!("* {}", ident);

                if let Some(doc) = field.documentation() {
                    print!(
                        ": {}",
                        doc.lines()
                            .map(|line| line.trim())
                            .filter(|line| !line.is_empty())
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }

                println!();
            }
            _ => unreachable!(),
        }
    }
}
