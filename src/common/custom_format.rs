use serde::{self, Deserialize, Deserializer, Serializer};
use time::{format_description, Date};

const FORMAT_STR: &str = "[year]-[month]-[day]";
// Custom serialization
pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = format_description::parse(FORMAT_STR).expect("Wrong format description!");
    let s = date.format(&format).expect("Date format error");
    serializer.serialize_str(&s)
}

// Custom deserialization
pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let format = format_description::parse(FORMAT_STR).expect("Wrong format description!");
    let s = String::deserialize(deserializer)?;
    Date::parse(&s, &format).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use crate::common::post::PostMetadata;
    use time::macros::date;

    #[test]
    fn test_my_struct_serialization_deserialization() {
        // Create an instance of MyStruct
        let my_struct = PostMetadata {
            title: "title".to_owned(),
            description: "description".to_owned(),
            tags: vec!["abc".to_owned()],
            publication_date: date!(2019 - 01 - 01),
        };

        // Serialize MyStruct
        let serialized = serde_json::to_string(&my_struct).expect("Failed to serialize");
        println!("{}", serialized);

        // Deserialize it back
        let deserialized: PostMetadata =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Assert that the deserialized instance matches the original
        assert_eq!(my_struct.publication_date, deserialized.publication_date);
    }
}
