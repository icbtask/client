use assert_cmd::Command;
use mockito::mock;
use pretty_assertions::assert_eq;

#[test]
fn create_address() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock("POST", "/address")
        .with_status(204)
        .with_header("content-type", "application/json")
        .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("address")
        .arg("add")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "New address created\n";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn list_addressess() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock("GET", "/addresses")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
    [
        {
            "address": "afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca",
            "todolist_id": null,
            "allowed_addresses": []
        }
    ]
            "#,
        )
        .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("address")
        .arg("list")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "\
+--------------------------------------------------+-------------+-------------------+
| address                                          | todolist_id | allowed_addresses |
+--------------------------------------------------+-------------+-------------------+
| afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca |             |                   |
+--------------------------------------------------+-------------+-------------------+
";
    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn delete_address() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock(
        "DELETE",
        "/address/afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca",
    )
    .with_status(204)
    .with_header("content-type", "application/json")
    .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("address")
        .arg("delete")
        .arg("--address=afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Address deleted\n";

    assert_eq!(output, expected);

    Ok(())
}
