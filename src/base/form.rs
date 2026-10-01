use deboa::{form::EncodedForm, TestResult};

pub fn test_encoded_form() -> TestResult<()> {
    let form = EncodedForm::builder()
        .field("name", "deboa")
        .field("version", "0.0.1");

    let form = form.build();

    assert_eq!(form.to_vec(), b"name=deboa&version=0.0.1");

    Ok(())
}
