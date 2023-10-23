use introspect::Entity;
use introspect::Introspect;
use introspect::Introspected;
use introspect::Member;

/// This is the documentation for the [`ExampleOne`] enum.
///
/// "Testing"
///
/// We can add more text down here.
#[allow(dead_code)]
#[derive(Introspect)]
enum ExampleOne {
    /// The first variant.
    One,

    /// The second variant.
    ///
    /// And some more text.
    Two,
}

#[allow(dead_code)]
#[derive(Introspect)]
enum ExampleTwo {
    One,
    Two,
}

pub fn main() {
    print_enum::<ExampleOne>();

    println!();

    print_enum::<ExampleTwo>();
}

fn print_enum<S: Introspected>() {
    let enum_ = match S::introspected_entity() {
        Entity::Enum(enum_) => enum_,
        _ => unreachable!(),
    };

    println!("[{}]\n", enum_.identifier());

    if let Some(documentation) = enum_.documentation() {
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

    for variant in S::introspected_members() {
        match variant {
            Member::Variant(variant) => {
                print!("* {}", variant.identifier());

                if let Some(doc) = variant.documentation() {
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
