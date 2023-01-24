//! ```rust
//! use pretty_type_name::pretty_type_name;
//!
//! mod foo {
//!     pub mod bar {
//!         pub struct X<T>(T);
//!     }
//!
//!     pub struct Y;
//! }
//!
//! println!("{}", pretty_type_name::<foo::bar::X<foo::Y>>());
//! # assert_eq!(pretty_type_name::<foo::bar::X<foo::Y>>(), "X<Y>".to_string());
//! // prints `X<Y>`
//! ```

/// Returns a shorter version of [`std::any::type_name`]
pub fn pretty_type_name<T: ?Sized>() -> String {
    let type_name = std::any::type_name::<T>();
    pretty_type_name_str(type_name)
}

/// same as [pretty_type_name], but works strings
pub fn pretty_type_name_str(type_name: &str) -> String {
    if let Some(before) = type_name.strip_suffix("::{{closure}}") {
        return format!("{}::{{{{closure}}}}", pretty_type_name_str(before));
    }

    // code taken from [bevy](https://github.com/bevyengine/bevy/blob/89a41bc62843be5f92b4b978f6d801af4de14a2d/crates/bevy_reflect/src/type_registry.rs#L156)
    let mut short_name = String::new();

    // A typename may be a composition of several other type names (e.g. generic parameters)
    // separated by the characters that we try to find below.
    // Then, each individual typename is shortened to its last path component.
    //
    // Note: Instead of `find`, `split_inclusive` would be nice but it's still unstable...
    let mut remainder = type_name;
    while let Some(index) = remainder.find(&['<', '>', '(', ')', '[', ']', ',', ';'][..]) {
        let (path, new_remainder) = remainder.split_at(index);
        // Push the shortened path in front of the found character
        short_name.push_str(path.rsplit(':').next().unwrap());
        // Push the character that was found
        let character = new_remainder.chars().next().unwrap();
        short_name.push(character);
        // Advance the remainder
        if character == ',' || character == ';' {
            // A comma or semicolon is always followed by a space
            short_name.push(' ');
            remainder = &new_remainder[2..];
        } else {
            remainder = &new_remainder[1..];
        }
    }

    // The remainder will only be non-empty if there were no matches at all
    if !remainder.is_empty() {
        // Then, the full typename is a path that has to be shortened
        short_name.push_str(remainder.rsplit(':').next().unwrap());
    }

    short_name
}

#[cfg(test)]
mod tests;
