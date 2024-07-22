use super::{project, template};

#[cfg(test)]
mod command_tests {
    use super::project::{create_project, DEBUG_PROJECTS_PATH};
    use super::template::DEBUG_TEMPLATES_PATH;
    use std::fs;
    use std::path::PathBuf;

    const TEST_TEMPLATE_NAME: &str = "test-template";
    const TEST_TEMPLATE_FILE: &str = "test-template-file.txt";
    const TEST_PROJECT_NAME: &str = "test-project";

    /// Setup a test template for use in tests.
    fn setup_test_template(suffix: &str) {
        // Setup
        let test_template_name = TEST_TEMPLATE_NAME.to_owned() + "_" + suffix;

        // Create a test template file
        let test_template_path = PathBuf::from(TEST_TEMPLATE_FILE);
        fs::write(&test_template_path, "This is a test template file.").unwrap();

        // Create another file in some nested directories in the template
        let nested_dir = PathBuf::from(DEBUG_TEMPLATES_PATH)
            .join(&test_template_name)
            .join("nested")
            .join("dir");
        fs::create_dir_all(&nested_dir).unwrap();
        let nested_file = nested_dir.join("nested-file.txt");
        fs::write(&nested_file, "This is a nested file.").unwrap();

        // Write the projx.toml file for the test template
        let test_template_dir = PathBuf::from(DEBUG_TEMPLATES_PATH).join(&test_template_name);
        fs::create_dir_all(&test_template_dir).unwrap();
        let projx_toml_path = test_template_dir.join("projx.toml");
        let projx_toml_contents = format!(
            "name = \"{}\"\ndescription = \"Test template.\"\nauthor = \"Test author.\"\n",
            test_template_name
        );
        fs::write(&projx_toml_path, projx_toml_contents).unwrap();

        // Copy the test template file to the test template directory
        let test_template_file = test_template_dir.join(TEST_TEMPLATE_FILE);
        fs::copy(&test_template_path, &test_template_file).unwrap();
    }

    /// Cleanup a test template after use in tests.
    fn cleanup_test_template(suffix: &str) {
        // Setup
        let test_template_name = TEST_TEMPLATE_NAME.to_owned() + "_" + suffix;

        // Remove the test template file
        let test_template_path = PathBuf::from(TEST_TEMPLATE_FILE);
        if test_template_path.exists() {
            fs::remove_file(test_template_path).unwrap();
        }

        // Remove the test template directory
        let test_template_dir = PathBuf::from(DEBUG_TEMPLATES_PATH).join(test_template_name);
        if test_template_dir.exists() {
            fs::remove_dir_all(test_template_dir).unwrap();
        }
    }

    /// Cleanup a test project after use in tests.
    fn cleanup_test_project(suffix: &str) {
        let test_project_name = TEST_PROJECT_NAME.to_owned() + "_" + suffix;

        // Remove the test project directory
        let test_project_dir = PathBuf::from(DEBUG_PROJECTS_PATH).join(test_project_name);
        if test_project_dir.exists() {
            fs::remove_dir_all(test_project_dir).unwrap();
        }
    }

    #[test]
    fn test_add_template() {
        // Setup
        println!("\x1b[33mWARNING: This test does not actually test add_template yet. It only tests the test helpers. A better test should be implemented when the add_template function no longer requires user prompts.\x1b[0m");
        let test_suffix = "add_template";
        setup_test_template(&test_suffix);

        // Verify that the template directory was created
        let test_template_dir = PathBuf::from(DEBUG_TEMPLATES_PATH)
            .join(TEST_TEMPLATE_NAME.to_owned() + "_" + &test_suffix);
        assert!(test_template_dir.exists());

        // Verify that the projx.toml file was created
        let projx_toml_path = test_template_dir.join("projx.toml");
        assert!(projx_toml_path.exists());

        cleanup_test_template(&test_suffix);
    }

    #[test]
    fn test_create_project() {
        // Setup
        let test_suffix = "create_project";
        let test_template_name = TEST_TEMPLATE_NAME.to_owned() + "_" + test_suffix;
        let test_project_name = TEST_PROJECT_NAME.to_owned() + "_" + test_suffix;
        setup_test_template(&test_suffix);

        // Create the project
        create_project(&test_template_name, &test_project_name, None).unwrap();

        // Verify that the project directory was created
        let test_project_dir = PathBuf::from(DEBUG_PROJECTS_PATH).join(test_project_name);
        assert!(test_project_dir.exists());

        //verify that the projx.toml file was copied to the project directory
        let projx_toml_path = test_project_dir.join("projx.toml");
        assert!(projx_toml_path.exists(), "projx.toml file does not exist.");

        // Verify that the project file was copied to the project directory
        let test_template_dir = PathBuf::from(DEBUG_TEMPLATES_PATH).join(test_template_name);
        let test_template_file = test_template_dir.join(TEST_TEMPLATE_FILE);
        let test_project_file = test_project_dir.join(TEST_TEMPLATE_FILE);
        assert!(test_project_file.exists(), "Project file does not exist.");

        // Verify that the contents of the test template file and the test project file are the same
        let test_template_contents = fs::read_to_string(&test_template_file).expect(
            format!(
                "Could not read test template file: {}",
                test_template_file.display()
            )
            .as_str(),
        );
        let test_project_contents = fs::read_to_string(&test_project_file).expect(
            format!(
                "Could not read test project file: {}",
                test_project_file.display()
            )
            .as_str(),
        );
        assert_eq!(
            test_template_contents, test_project_contents,
            "Template and project file contents do not match."
        );

        let test_filepaths_from_test_root = fs::read_dir(&test_project_dir)
            .unwrap()
            .map(|entry| {
                entry
                    .unwrap()
                    .path()
                    .strip_prefix(&test_project_dir)
                    .unwrap()
                    .to_path_buf()
            })
            .collect::<Vec<PathBuf>>();
        let test_filepaths_from_test_template = fs::read_dir(&test_template_dir)
            .unwrap()
            .map(|entry| {
                entry
                    .unwrap()
                    .path()
                    .strip_prefix(&test_template_dir)
                    .unwrap()
                    .to_path_buf()
            })
            .collect::<Vec<PathBuf>>();
        assert_eq!(
            test_filepaths_from_test_root, test_filepaths_from_test_template,
            "Files in project directory do not match files in template directory."
        );

        cleanup_test_project(&test_suffix);
        cleanup_test_template(&test_suffix);
    }
}

#[cfg(test)]
mod template_tests {
    use super::template::Template;
    use std::{fs, path::PathBuf};

    /// Test to ensure we can load our first ever template into the Template struct
    ///
    #[test]
    fn test_load_nodejs_template() {
        let template_path = PathBuf::from("src/templates/nodejs-webapp");
        let template = Template::load(&template_path).expect("Failed to load template");

        assert_eq!(
            template.name, "nodejs-webapp",
            "Template name does not match."
        );
        assert_eq!(
            template.description, "A simple express web app",
            "Template description does not match."
        );
        assert_eq!(
            template.author, "Isaac Gordon",
            "Template author does not match."
        );
        assert!(
            template
                .files
                .iter()
                .any(|file| file.ends_with("projx.toml")),
            "projx.toml file is missing."
        );
        assert!(
            template.files.iter().any(|file| file.ends_with("app.js")),
            "app.js file is missing."
        );
        assert!(
            template
                .files
                .iter()
                .any(|file| file.ends_with("package.json")),
            "package.json file is missing."
        );
    }

    /// Test that any and all templates in the repo's templates directory be loaded.
    /// We will use this test to ensure that all templates are valid and can be loaded.
    ///
    /// This test will fail if any template fails to load.
    #[test]
    fn test_all_templates_load() {
        let templates_root = PathBuf::from("src/templates");
        let template_dirs = fs::read_dir(&templates_root)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<PathBuf>>();

        let mut load_errors = Vec::new();
        for template_dir in template_dirs {
            match Template::load(&template_dir) {
                Ok(template) => {
                    println!("Loaded template: {}", template.name);
                }
                Err(err) => {
                    eprintln!("Failed to load template: {:?}", err);
                    load_errors.push(err);
                }
            }
        }

        assert!(
            load_errors.is_empty(),
            "Failed to load templates: {:?}",
            load_errors
        );
    }
}
