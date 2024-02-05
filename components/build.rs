use std::{env, fs, io::Write, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=src/client");
    println!("cargo:rerun-if-changed=../web-htmx/src");

    println!("Building client scripts {:?}", std::time::SystemTime::now());

    let out_dir = env::var("OUT_DIR").unwrap();
    let hash = hash_package_json();

    let should_bun_install = match read_hash_from_out(&out_dir) {
        Ok(old_hash) => old_hash != hash,
        Err(_) => true,
    };

    if should_bun_install {
        write_hash_to_out(&out_dir, &hash);
        exec_bun_install();
    }

    exec_build_js_and_css();
    println!("Built client scripts {:?}", std::time::SystemTime::now());
}

fn exec_bun_install() {
    let output = Command::new("sh")
        .args(["-c", "bun install"])
        .output()
        .expect("failed to execute `bun install`");

    println!("{:?}", output);
}

fn exec_build_js_and_css() {
    // TODO! Sourcemaps, minification, etc - handle differently for production.
    let profile = env::var("PROFILE").unwrap();
    let minify_flag = if profile == "release" { "--minify" } else { "" };

    let css_build = format!(
        "bunx tailwindcss -i ./src/client/common.css -o ./out/common.css {}",
        minify_flag
    );

    let js_build = format!(
        "bun build ./src/client/common.js --outdir ./out --sourcemap=external {}",
        minify_flag
    );
    let build_cmd = format!("{} && {}", css_build.trim(), js_build.trim());

    let output = Command::new("sh")
        .args(["-c", build_cmd.as_str()])
        .output()
        .expect("failed to execute tailwind and bun build process");

    println!("{:?}", output);
}

fn read_hash_from_out(out_dir: &str) -> Result<String, std::io::Error> {
    let path = format!("{}/package_json.txt", out_dir);
    fs::read_to_string(path)
}

fn write_hash_to_out(out_dir: &str, hash: &str) {
    let file_path = format!("{}/package_json.txt", out_dir);
    let mut file = fs::File::create(file_path).unwrap();
    write!(file, "{}", hash).unwrap();
}

fn hash_package_json() -> String {
    let bytes = fs::read("package.json").unwrap();
    sha256::digest(bytes)
}
