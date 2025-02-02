bareops_integration_test::test!(it_can_create_a_file, r#"
        task "create_my_file" {
            create_file {
                path: "/foo"
            }
        }
        "#, sh r#"
            ls -al /foo
        "#);
