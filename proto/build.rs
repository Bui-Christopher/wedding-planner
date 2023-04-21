fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(
            "wedding.Guest",
            "#[derive(poem_openapi::Object, serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "wedding.Goal",
            "#[derive(poem_openapi::Object, serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "wedding.Image",
            "#[derive(poem_openapi::Object, serde::Serialize, serde::Deserialize)]",
        )
        .field_attribute(
            "wedding.Guest.id",
            "#[oai(default = \"crate::common::get_new_uuid\")]",
        )
        .field_attribute(
            "wedding.Guest.food_preferences",
            "#[oai(default = \"String::new\")]",
        )
        .field_attribute(
            "wedding.Guest.song_requests",
            "#[oai(default = \"String::new\")]",
        )
        .field_attribute(
            "wedding.Goal.id",
            "#[oai(default = \"crate::common::get_new_uuid\")]",
        )
        .field_attribute(
            "wedding.Goal.progress_amt",
            "#[oai(default = \"i32::default\")]",
        )
        .field_attribute(
            "wedding.Image.id",
            "#[oai(default = \"crate::common::get_new_uuid\")]",
        )
        .compile(&["service.proto", "wedding.proto"], &["."])?;
    Ok(())
}
