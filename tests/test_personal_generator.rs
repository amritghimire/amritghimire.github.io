use personal_frontend::personal::generator;

#[test]
fn post_generator() {
    let generator = generator::Generator::new();
    assert!(!generator.experiences().is_empty());
    assert!(!generator.projects().is_empty());
    assert!(!generator.education().is_empty());
    assert!(!generator.certifications().is_empty());
}
