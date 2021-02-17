pub fn pretty_type_name<T: ?Sized>() -> String {
    let type_name = std::any::type_name::<T>();
    pretty_type_name_str(type_name)
}

pub fn pretty_type_name_str(type_name: &str) -> String {
    // fn types
    if type_name.starts_with("fn(") {
        if let Some((before, in_between, after)) = split_between(type_name, '(', ')') {
            debug_assert_eq!(before, "fn");
            debug_assert!(after.is_empty());
            let list = join_with(in_between.split(", ").map(pretty_type_name_str));
            return format!("fn({})", list);
        }
    }

    // handle tuple structs separately
    if let Some(inner) = type_name
        .strip_prefix('(')
        .and_then(|name| name.strip_suffix(')'))
    {
        if inner.contains('(') {
            return type_name.to_string(); // nested tuples are not supported yet
        }
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
    let (before, rest) = split_once(input, left_terminator)?;
    let (in_between, after) = rsplit_once(rest, right_terminator)?;
    Some((before, in_between, after))
}

// replace with `str_split_once` once stable (rust 1.51)
fn split_once(input: &str, delimiter: char) -> Option<(&str, &str)> {
    let mut iter = input.splitn(2, delimiter);
    Some((iter.next()?, iter.next()?))
}
fn rsplit_once(input: &str, delimiter: char) -> Option<(&str, &str)> {
    let mut iter = input.rsplitn(2, delimiter);
    let rightmost = iter.next()?;
    let rest = iter.next()?;
    Some((rest, rightmost))
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
