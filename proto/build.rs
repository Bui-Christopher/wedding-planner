fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(
            "objects.Guest",
            "#[derive(poem_openapi::Object, serde::Serialize, serde::Deserialize, scylla::macros::FromRow, scylla::ValueList)]",
        )
        .type_attribute(
            "objects.Goal",
            "#[derive(poem_openapi::Object, serde::Serialize, serde::Deserialize, scylla::macros::FromRow, scylla::ValueList)]",
        )
        .type_attribute(
            "objects.Image",
            "#[derive(poem_openapi::Object, serde::Serialize, serde::Deserialize, scylla::macros::FromRow, scylla::ValueList)]",
        )
        .field_attribute(
            "objects.Guest.id",
            "#[oai(default = \"crate::common::get_new_uuid\")]",
        )
        .field_attribute(
            "objects.Guest.first_name",
            "#[oai(default = \"String::new\")]",
        )
        .field_attribute(
            "objects.Guest.last_name",
            "#[oai(default = \"String::new\")]",
        )
        .field_attribute("objects.Guest.address", "#[oai(default = \"String::new\")]")
        .field_attribute("objects.Guest.email", "#[oai(default = \"String::new\")]")
        .field_attribute(
            "objects.Guest.food_preferences",
            "#[oai(default = \"String::new\")]",
        )
        .field_attribute(
            "objects.Guest.song_requests",
            "#[oai(default = \"String::new\")]",
        )
        .field_attribute(
            "objects.Goal.id",
            "#[oai(default = \"crate::common::get_new_uuid\")]",
        )
        .field_attribute(
            "objects.Goal.progress_amt",
            "#[oai(default = \"i32::default\")]",
        )
        .field_attribute(
            "objects.Image.id",
            "#[oai(default = \"crate::common::get_new_uuid\")]",
        )
        .field_attribute(
            "objects.Image.filename",
            "#[oai(default = \"String::new\")]",
        )
        .field_attribute(
            "objects.Image.extension",
            "#[oai(default = \"String::new\")]",
        )
        .field_attribute(
            "entity.Image.content",
            "#[serde(with = \"serde_hex::SerHexSeq::<serde_hex::StrictPfx>\")]",
        )
        .compile(&["methods.proto", "objects.proto"], &["."])?;
    Ok(())
}
