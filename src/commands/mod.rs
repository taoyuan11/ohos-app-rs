pub mod build;
pub mod init;
pub mod package;

use std::collections::BTreeMap;

use crate::config::AppContext;

pub(crate) fn command_env(app: &AppContext) -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();
    env.insert(
        "DEVECO_STUDIO_HOME".to_string(),
        app.config.deveco_studio_dir.display().to_string(),
    );
    env.insert(
        "HARMONY_APP_SDK_ROOT".to_string(),
        app.sdk.root.display().to_string(),
    );
    env.insert(
        "OHOS_BASE_SDK_HOME".to_string(),
        app.sdk.root.display().to_string(),
    );
    env.insert(
        "HARMONY_APP_SDK_VERSION".to_string(),
        app.sdk.version.clone(),
    );
    env.insert(
        "OHOS_SDK_HOME".to_string(),
        app.sdk.version_dir.display().to_string(),
    );
    env.insert(
        "DEVECO_SDK_HOME".to_string(),
        app.sdk.root.display().to_string(),
    );
    env.insert(
        "OHOS_SDK_NATIVE".to_string(),
        app.sdk.native_dir.display().to_string(),
    );
    env.insert(
        "OHOS_SDK_TOOLCHAINS".to_string(),
        app.sdk.toolchains_dir.display().to_string(),
    );
    let clang_bin = app
        .sdk
        .native_dir
        .join("llvm")
        .join("bin")
        .join("clang.exe");
    let cxx = app
        .sdk
        .native_dir
        .join("llvm")
        .join("bin")
        .join("clang++.exe");
    let ar = app
        .sdk
        .native_dir
        .join("llvm")
        .join("bin")
        .join("llvm-ar.exe");
    let normalized_target = app.config.target.replace('-', "_");
    let clang_target = clang_target(&app.config.target);
    let sysroot = app.sdk.native_dir.join("sysroot");
    env.insert(
        format!(
            "CARGO_TARGET_{}_LINKER",
            normalized_target.to_ascii_uppercase()
        ),
        clang_bin.display().to_string(),
    );
    env.insert(
        "RUSTFLAGS".to_string(),
        format!(
            "-Clink-arg=--target={clang_target} -Clink-arg=--sysroot={} -Clink-arg=-D__MUSL__",
            sysroot.display()
        ),
    );
    env.insert(
        format!("CC_{normalized_target}"),
        clang_bin.display().to_string(),
    );
    env.insert(
        format!("CXX_{normalized_target}"),
        cxx.display().to_string(),
    );
    env.insert(format!("AR_{normalized_target}"), ar.display().to_string());
    env
}

fn clang_target(rust_target: &str) -> String {
    match rust_target {
        "aarch64-unknown-linux-ohos" => "aarch64-linux-ohos".to_string(),
        "armv7-unknown-linux-ohos" => "arm-linux-ohos".to_string(),
        "x86_64-unknown-linux-ohos" => "x86_64-linux-ohos".to_string(),
        "loongarch64-unknown-linux-ohos" => "loongarch64-linux-ohos".to_string(),
        _ => rust_target.to_string(),
    }
}
