#![feature(str_split_once)]

pub fn pretty_type_name<T: ?Sized>() -> String {
    let type_name = std::any::type_name::<T>();
    pretty_type_name_str(type_name)
}

pub(crate) fn pretty_type_name_str(type_name: &str) -> String {
    // handle tuple structs separately
    if let Some(inner) = type_name
        .strip_prefix('(')
        .and_then(|name| name.strip_suffix(')'))
    {
        let list = join_with(inner.split(", ").map(pretty_type_name_str));
        return format!("({})", list);
    }

    if let Some((before, in_between, after)) = split_between(type_name, '<', '>') {
        let before = last_after(before, "::");
        return format!("{}<{}>{}", before, pretty_type_name_str(in_between), after);
    }

    last_after(type_name, "::").to_string()
}

fn last_after<'a>(input: &'a str, delimiter: &str) -> &'a str {
    input.rsplit(delimiter).next().unwrap_or(input)
}

fn split_between(
    input: &str,
    left_terminator: char,
    right_terminator: char,
) -> Option<(&str, &str, &str)> {
    let (before, rest) = input.split_once(left_terminator)?;
    let (in_between, after) = rest.rsplit_once(right_terminator)?;
    Some((before, in_between, after))
}

fn join_with(iter: impl Iterator<Item = String>) -> String {
    let mut iter = iter.peekable();

    let mut string = String::new();
    while let Some(item) = iter.next() {
        string.push_str(&item);
        if iter.peek().is_some() {
            string.push_str(", ");
        }
    }
    string
}

#[cfg(test)]
mod tests;
