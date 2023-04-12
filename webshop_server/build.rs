use static_files::resource_dir;

const DEFAULT_REACT_DIST: &str = "../webshop_frontend/dist";
const DUMMY_OUT_DIR: &str = "./target/debug/build/webshop_server-dummy-hmtl";

fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let react_dist = std::env::var("FRONT_DIST_DIR").unwrap_or_else(|_| DEFAULT_REACT_DIST.to_string());

    match resource_dir(react_dist).build() {
        Ok(_) => {
            println!("cargo:warning=\x1b[31m INCLUDED PREBUILT REACT \x1b[0m");
        }
        Err(_) => {
            if !cfg!(debug_assertions) {
                panic!("Cannot build in release mode without prebuilt react. Run npm run build in webshop_frontend");
            } else {
                println!("cargo:warning= \x1b[31m PREBUILT REACT NOT FOUND \x1b[0m");
                println!("cargo:warning= \x1b[31m BULIDING DUMMY FILE \x1b[0m");
                std::fs::create_dir_all(DUMMY_OUT_DIR).expect("Failed to create dummy dir");
                std::fs::write(DUMMY_OUT_DIR.to_string() + "/index.html", dummy_hmtl())
                    .expect("Failed to create dummy file");
                resource_dir(DUMMY_OUT_DIR).build()?;
            }
        }
    }
    Ok(())
}

fn dummy_hmtl() -> String {
    String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Webshop dummy</title>
    <style>
        emp {font-weight: bold;}
    </style>
</head>
<body>
    <h1>Dummy hmtl</h1>
    <p>Server built without React.</p>
    <p>Run <emp>npm run build</emp> in <emp>webshop_frontend</emp> to build the React app.</p>
</body>
</html>
"#,
    )
}
