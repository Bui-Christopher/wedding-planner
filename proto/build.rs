fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute("wedding.Guest", "#[derive(poem_openapi::Object, serde::Serialize, serde::Deserialize)]")
        .type_attribute("wedding.Goal", "#[derive(poem_openapi::Object, serde::Serialize, serde::Deserialize)]")
        .type_attribute("wedding.Image", "#[derive(poem_openapi::Object, serde::Serialize, serde::Deserialize)]")
        .compile(&["wedding.proto"], &["."])?;
    Ok(())
}
