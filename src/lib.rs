use std::panic;

use byond::byond;

macro_rules! catch_panics {
    ( $e:expr ) => {{
        let res = panic::catch_unwind(|| $e);

        match res {
            Ok(r) => r,
            Err(e) => match e.downcast::<String>() {
                Ok(e) => *e,
                Err(e) => match e.downcast::<&str>() {
                    Ok(e) => e.to_string(),
                    Err(_) => String::from("Unknown error"),
                },
            },
        }
    }};
}

fn to_yaml_inner(json: &str) -> String {
    let json = serde_json::from_str::<serde_json::Value>(json).unwrap();

    serde_yaml::to_string(&json).unwrap()
}

fn from_yaml_inner(yaml: &str) -> String {
    let yaml = serde_yaml::from_str::<serde_json::Value>(yaml).unwrap();

    serde_json::to_string(&yaml).unwrap()
}

fn to_toml_inner(json: &str) -> String {
    let json = serde_json::from_str::<serde_json::Value>(json).unwrap();

    toml::to_string(&json).unwrap()
}

fn from_toml_inner(toml: &str) -> String {
    let toml = toml::from_str::<toml::Value>(toml).unwrap();

    serde_json::to_string(&toml).unwrap()
}

byond!(to_yaml: json; {
    catch_panics!(to_yaml_inner(json))
});

byond!(from_yaml: yaml; {
    catch_panics!(from_yaml_inner(yaml))
});

byond!(to_toml: json; {
    catch_panics!(to_toml_inner(json))
});

byond!(from_toml: toml; {
    catch_panics!(from_toml_inner(toml))
});

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{from_toml_inner, from_yaml_inner, to_toml_inner, to_yaml_inner};

    #[test]
    fn test_to_yaml() {
        let initial = r#"{
    "data": "Some data"
}"#;

        let yaml = to_yaml_inner(initial);

        assert_eq!(
            yaml,
            r#"---
data: Some data
"#
        )
    }

    #[test]
    fn test_from_yaml() {
        let initial = r#"---
data: Some data
"#;

        let json = from_yaml_inner(initial);

        assert_eq!(json, r#"{"data":"Some data"}"#)
    }

    #[test]
    fn test_to_toml() {
        let initial = r#"{
"data": "Some data"
}"#;

        let toml = to_toml_inner(initial);

        assert_eq!(
            toml,
            r#"data = "Some data"
"#
        )
    }

    #[test]
    fn test_from_toml() {
        let initial = r#"data = "Some data""#;

        let json = from_toml_inner(initial);

        assert_eq!(json, r#"{"data":"Some data"}"#)
    }
}
