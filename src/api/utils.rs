use std::collections::HashMap;

use tracing::info;

pub fn create_hashmap<K, V>(pairs: &[(K, V)]) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    let mut map = HashMap::new();
    for (k, v) in pairs {
        map.insert(k.clone(), v.clone());
    }
    map
}

pub fn prompt_open_url(url: String) -> std::io::Result<()> {
    info!("Open this link in your web browser to continue:\n\n{}\n", url);
    open::that(url)
}

#[macro_export]
macro_rules! serde_str_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $variant:ident = $string:literal,
            )*
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $variant,
            )*
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let s = match self {
                    $(
                        $name::$variant => $string,
                    )*
                };
                serializer.serialize_str(s)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct $nameVisitor;

                impl<'de> serde::de::Visitor<'de> for $nameVisitor {
                    type Value = $name;
        
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("A comma-separated list of valid search types")
                    }
        
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            $(
                                $string => Ok($name::$variant),
                            )*
                            _ => Err(E::unknown_variant(value, &[$($string),*])),
                        }
                    }
                }
            }
        }
    }
}