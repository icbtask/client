use assert_cmd::Command;
use mockito::mock;
use pretty_assertions::assert_eq;

#[test]
fn create_todolist() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock("POST", "/todolist")
        .with_status(204)
        .with_header("content-type", "application/json")
        .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("todolist")
        .arg("add")
        .arg("--name=My Todolist")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "New todolist `My Todolist` created\n";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn list_todolist() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock("GET", "/todolists")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
            [
        {
            "created_at": "2022-01-03T10:00:00",
            "name": "My Todolist",
            "todolist_id": "11b67ff3"
        }
    ]
            "#,
        )
        .create();
    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("todolist")
        .arg("list")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "\
+----------+-------------+
| id       | name        |
+----------+-------------+
| 11b67ff3 | My Todolist |
+----------+-------------+
";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn delete_todolist() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock("DELETE", "/todolist/11b67ff3")
        .with_status(204)
        .with_header("content-type", "application/json")
        .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("todolist")
        .arg("delete")
        .arg("--todolist-id=11b67ff3")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Todolist deleted\n";

    assert_eq!(output, expected);

    Ok(())
}
