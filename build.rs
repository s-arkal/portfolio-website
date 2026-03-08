fn main() {
    // Tell Cargo that if the given env var changes, to rerun this build script.
    // This ensures that `option_env!` in the code will always pick up the latest 
    // Cloudflare environment variable, avoiding aggressive caching issues.
    println!("cargo::rerun-if-env-changed=WEB3FORMS_ACCESS_KEY");
}
