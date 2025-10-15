#[test]
fn test_descriptors_toml_valid() {
    let toml_content =
        std::fs::read_to_string("data/descriptors.toml").expect("Failed to read descriptors.toml");

    let result: Result<numenera_chargen::data::models::DescriptorsData, _> =
        toml::from_str(&toml_content);

    assert!(
        result.is_ok(),
        "Failed to parse descriptors.toml: {:?}",
        result.err()
    );
}

#[test]
fn test_foci_toml_valid() {
    let toml_content = std::fs::read_to_string("data/foci.toml").expect("Failed to read foci.toml");

    let result: Result<numenera_chargen::data::models::FociData, _> = toml::from_str(&toml_content);

    assert!(
        result.is_ok(),
        "Failed to parse foci.toml: {:?}",
        result.err()
    );

    let data = result.unwrap();
    assert_eq!(data.foci.len(), 51, "Should have exactly 51 foci"); // Fixed!
}

#[test]
fn test_types_toml_valid() {
    let toml_content =
        std::fs::read_to_string("data/types.toml").expect("Failed to read types.toml");

    let result: Result<numenera_chargen::data::models::TypesData, _> =
        toml::from_str(&toml_content);

    assert!(
        result.is_ok(),
        "Failed to parse types.toml: {:?}",
        result.err()
    );
}
