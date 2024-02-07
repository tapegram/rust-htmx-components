use std::collections::HashMap;

pub fn opt_attr<S: AsRef<str>, T: AsRef<str>>(key: S, val: T) -> String {
    if val.as_ref().is_empty() {
        String::from("")
    } else {
        format!("{}=\"{}\"", key.as_ref(), val.as_ref())
    }
}

pub fn opt_attrs<S: AsRef<str>, T: AsRef<str>>(map: HashMap<S, T>) -> String {
    if map.is_empty() {
        String::from("")
    } else {
        let mut attrs = map
            .iter()
            .map(|(key, val)| opt_attr(key, val))
            .collect::<Vec<String>>();

        // Output attributes in alpha order.
        attrs.sort();
        attrs.join(" ").trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_opt_attr_with_empty_string() {
        assert_eq!(opt_attr("foo", ""), String::from(""));
    }

    #[test]
    fn test_opt_attr_with_non_empty_string() {
        assert_eq!(
            opt_attr("foo", String::from("baz")),
            String::from("foo=\"baz\"")
        );
    }

    #[test]
    fn test_opt_attr_with_string_with_spaces() {
        assert_eq!(
            opt_attr("foo", String::from("foo bar baz")),
            String::from("foo=\"foo bar baz\"")
        );
    }

    #[test]
    fn test_opt_attrs_with_empty_map() {
        assert_eq!(opt_attrs(HashMap::<&str, &str>::new()), String::from(""));
    }

    #[test]
    fn test_opt_attrs_with_empty_map_empty_array() {
        assert_eq!(opt_attrs(HashMap::<&str, &str>::from([])), String::from(""));
    }

    #[test]
    fn test_opt_attrs_with_single_attr_that_is_empty() {
        assert_eq!(opt_attrs(HashMap::from([("foo", "")])), String::from(""));
    }

    #[test]
    fn test_opt_attrs_with_multiple_attrs_that_are_empty() {
        assert_eq!(
            opt_attrs(HashMap::from([
                ("foo", String::from("")),
                ("baz", String::from("")),
            ])),
            String::from("")
        );
    }

    #[test]
    fn test_opt_attrs_with_single_attribute_tuple() {
        assert_eq!(
            opt_attrs(HashMap::from([("foo", String::from("baz"))])),
            String::from("foo=\"baz\"")
        );
    }

    #[test]
    fn test_opt_attrs_with_multiple_attribute_tuple() {
        let attrs = opt_attrs(HashMap::from([
            ("foo", String::from("baz")),
            ("bar", String::from("fuzz fuzz-baz")),
        ]));

        assert_eq!(attrs, String::from("bar=\"fuzz fuzz-baz\" foo=\"baz\""),);
    }
}
