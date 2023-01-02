use assert_cmd::Command;
use mockito::mock;
use pretty_assertions::assert_eq;

#[test]
fn create_task() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock("POST", "/task")
        .with_status(204)
        .with_header("content-type", "application/json")
        .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("task")
        .arg("add")
        .arg("--todolist_id=11b67ff3")
        .arg("--project=Tools")
        .arg("--description=Build new todolist tool")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "New task created\n";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn list_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock("GET", "/tasks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
    [
        {
            "created_at": "2022-01-03T10:00:00",
            "description": "Build new todolist tool",
            "project": "Tools",
            "shared_from": [],
            "shared_with": [],
            "status": "created",
            "task_id": "12345678",
            "todolist": {
                "created_at": "2022-01-03T10:00:00",
                "todolist_id": "99999999",
                "name": "My Todolist"
            },
            "updated_at": "2022-01-03T10:00:00"
        }
    ]
            "#,
        )
        .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("task")
        .arg("list")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "\
+----------+---------+-------------------------+
|                 My Todolist                  |
+----------+---------+-------------------------+
|    id    | project |       description       |
+----------+---------+-------------------------+
| 12345678 |  Tools  | Build new todolist tool |
+----------+---------+-------------------------+
";
    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn delete_task() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock("DELETE", "/task/11b67ff3")
        .with_status(204)
        .with_header("content-type", "application/json")
        .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("task")
        .arg("delete")
        .arg("--id=11b67ff3")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Task deleted\n";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn complete_task() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock("PATCH", "/task/11b67ff3")
        .with_status(204)
        .with_header("content-type", "application/json")
        .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("task")
        .arg("complete")
        .arg("--id=11b67ff3")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Task completed\n";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn share_task() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock(
        "POST",
        "/task/share/11b67ff3/afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca",
    )
    .with_status(204)
    .with_header("content-type", "application/json")
    .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("task")
        .arg("share")
        .arg("--id=11b67ff3")
        .arg("--address=afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Task shared\n";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn unshare_task() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock(
        "DELETE",
        "/task/share/11b67ff3/afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca",
    )
    .with_status(204)
    .with_header("content-type", "application/json")
    .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("task")
        .arg("unshare")
        .arg("--id=11b67ff3")
        .arg("--address=afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Task unshared\n";

    assert_eq!(output, expected);

    Ok(())
}
