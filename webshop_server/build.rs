use static_files::resource_dir;
use std::process::Command;

const FRONTEND_DIR: &str = "../webshop_frontend/src";
const REACT_OUT_DIR: &str = "../webshop_frontend/dist";
const DUMMY_OUT_DIR: &str = "./target/debug/build/webshop_server-dummy-hmtl";

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed={}", FRONTEND_DIR);
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-env-changed=BUILD_REACT");

    println!("cargo:warning= Running buil script...");

    dotenvy::dotenv().ok();
    let build_react = std::env::var("BUILD_REACT").unwrap_or("false".to_string());

    // If BUILD_REACT=true, or if build in release mode, build the react app
    if build_react.eq_ignore_ascii_case("true") || !cfg!(debug_assertions) {
        println!("cargo:warning=\x1b[31m BULIDING REACT \x1b[0m");
        let (program, c_option) = match cfg!(target_os = "windows") {
            true => ("cmd", "/C"), // windows
            false => ("sh", "-c"), // unix
        };
        let status = Command::new(program)
            .args(&[c_option, "npm run build"])
            .current_dir(FRONTEND_DIR)
            .status()
            .expect("Failed to run npm");
        assert!(status.success());
        resource_dir(REACT_OUT_DIR).build()?;
    } else {
        match resource_dir(REACT_OUT_DIR).build() {
            Ok(_) => {
                println!("cargo:warning=\x1b[31m INCLUDED PREBUILT REACT \x1b[0m");
            }
            Err(_) => {
                println!("cargo:warning= \x1b[31m PREBUILT REACT NOT FOUND \x1b[0m");
                println!("cargo:warning= \x1b[31m BULIDING DUMMY FILE \x1b[0m");
                std::fs::create_dir_all(DUMMY_OUT_DIR).expect("Failed to create dummy dir");
                std::fs::write(DUMMY_OUT_DIR.to_string() + "/index.html", dummy_hmtl())
                    .expect("Failed to create dummy file");
                resource_dir(DUMMY_OUT_DIR).build()?;
            }
        }
    }
    println!("cargo:warning=Build script done.");
    Ok(())
}

fn dummy_hmtl() -> String {
    String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Webshop</title>
    <style>
        emp {font-weight: bold;}
    </style>
</head>
<body>
    <h1>Dummy hmtl</h1>
    <p>Server built without React. In order to build with React, do one of the following:</p>
    <ul>
        <li>Set <emp>BUILD_REACT</emp> environment variable to <emp>true</emp></li>
        <li>Set <emp>BUILD_REACT=true</emp> in <emp>.env</emp></li>
        <li>Build in release mode; run <emp>cargo build --release</emp></li>
    </ul>
</body>
</html>
"#,
    )
}
