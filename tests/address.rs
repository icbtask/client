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
            "todolist": null,
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

#[test]
fn attach_address() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock(
        "POST",
        "/address/todolist/afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca/11b67ff3",
    )
    .with_status(204)
    .with_header("content-type", "application/json")
    .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("address")
        .arg("attach")
        .arg("--todolist-id=11b67ff3")
        .arg("--address=afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Address attached successfuly\n";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn detach_address() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock(
        "DELETE",
        "/address/todolist/afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca",
    )
    .with_status(204)
    .with_header("content-type", "application/json")
    .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("address")
        .arg("detach")
        .arg("--address=afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Address detached successfuly\n";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn allow_address() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock(
        "POST",
        "/address/access/afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca/qaq7l7f6uutpqohbhqjqlr7jwnhmbrm43nj4p2pbg2qzsfs5rkua"
    )
    .with_status(204)
    .with_header("content-type", "application/json")
    .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("address")
        .arg("allow")
        .arg("--address=afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca")
        .arg("--remote-address=qaq7l7f6uutpqohbhqjqlr7jwnhmbrm43nj4p2pbg2qzsfs5rkua")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Remote address allowed\n";

    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn revoke_address() -> Result<(), Box<dyn std::error::Error>> {
    let url = &mockito::server_url();
    let _m = mock(
        "DELETE",
        "/address/access/afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca/qaq7l7f6uutpqohbhqjqlr7jwnhmbrm43nj4p2pbg2qzsfs5rkua"
    )
    .with_status(204)
    .with_header("content-type", "application/json")
    .create();

    let cmd = Command::cargo_bin("icbtask")?
        .env("API_KEY", "SECRETKEY")
        .env("BASE_URL", url)
        .arg("address")
        .arg("revoke")
        .arg("--address=afyp675e4wngq3qhiqyqqgeticgne4o2hxlsc3onztdhnbca")
        .arg("--remote-address=qaq7l7f6uutpqohbhqjqlr7jwnhmbrm43nj4p2pbg2qzsfs5rkua")
        .output()
        .unwrap();

    assert!(cmd.status.success());

    let output = String::from_utf8(cmd.stdout)?;

    let expected = "Remote address revoked\n";

    assert_eq!(output, expected);

    Ok(())
}
