use cassette;

const INPUT_1:&str = r#"
page
    tag
        .type "h1"
        "header text""#;

const OUTPUT_1: &str = "<h1>header text</h1>";

#[test]
fn check_basic_tag() {
    let project = cassette::parse_str(INPUT_1);
    println!("{:?}", project);

    assert!(project.is_ok());
    let project = project.unwrap();

    assert_eq!(project.documents.len(), 1);
    
    for document in project.documents {
        let output = document.to_string();

        assert!(output.is_ok());
        assert_eq!(output.unwrap(), OUTPUT_1);
    }
}
