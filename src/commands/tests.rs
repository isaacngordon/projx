use super::template;
use super::project;

#[cfg(test)]
mod tests {
    use super::template::add_template;
    use super::project::create_project;
    use std::fs;
    use std::path::PathBuf;

    const TEST_TEMPLATE_NAME: &str = "test-template";
    const TEST_TEMPLATE_FILE: &str = "test-template-file.txt";
    const TEST_PROJECT_NAME: &str = "test-project";

    fn setup_test_template() {
        // Create a test template file
        let test_template_path = PathBuf::from(TEST_TEMPLATE_FILE);
        fs::write(&test_template_path, "This is a test template file.").unwrap();

        // Add the template
        add_template(TEST_TEMPLATE_NAME, Some(TEST_TEMPLATE_FILE), None);
    }

    fn cleanup_test_template() {
        // Remove the test template file
        let test_template_path = PathBuf::from(TEST_TEMPLATE_FILE);
        if test_template_path.exists() {
            fs::remove_file(test_template_path).unwrap();
        }

        // Remove the test template directory
        let test_template_dir = PathBuf::from("src/templates").join(TEST_TEMPLATE_NAME);
        if test_template_dir.exists() {
            fs::remove_dir_all(test_template_dir).unwrap();
        }
    }

    fn cleanup_test_project() {
        // Remove the test project directory
        let test_project_dir = PathBuf::from(".projx/projects").join(TEST_PROJECT_NAME);
        if test_project_dir.exists() {
            fs::remove_dir_all(test_project_dir).unwrap();
        }
    }

    #[test]
    fn test_add_template() {
        setup_test_template();

        // Verify that the template directory was created
        let test_template_dir = PathBuf::from("src/templates").join(TEST_TEMPLATE_NAME);
        assert!(test_template_dir.exists());

        // Verify that the projx.toml file was created
        let projx_toml_path = test_template_dir.join("projx.toml");
        assert!(projx_toml_path.exists());

        cleanup_test_template();
    }

    #[test]
    fn test_create_project() {
        setup_test_template();

        // Create the project
        create_project(TEST_TEMPLATE_NAME, TEST_PROJECT_NAME, None);

        // Verify that the project directory was created
        let test_project_dir = PathBuf::from(".projx/projects").join(TEST_PROJECT_NAME);
        assert!(test_project_dir.exists());

        cleanup_test_project();
        cleanup_test_template();
    }
}
