// rust的环境变量库
use std::env;
use std::process::Command;

fn main() {
    println!("基于build.rs构建openssl的动态链接库");

    // 设置openssl的路径,主要用于编译openssl使用
    // to_string_lossy函数:工作原理是将输入的字符串切片转换为String类型的字符串，
    // 并将无法转换为有效UTF-8字符的部分替换为Unicode替代字符（U+FFFD）。
    let cwd: String = env::current_dir().unwrap().to_string_lossy().to_string();
    let openssl_dir: String = format!("{}/openssl-1.0.1f", cwd);

    println!("{}", openssl_dir);

    // 配置make clean
    Command::new("make")
        .arg("clean")
        .current_dir(openssl_dir.clone())
        .status()
        .expect("make clean openssl失败");

    // 配置.config
    Command::new("./config")
        .env("CC", "afl-clang-fast")
        .env("CXX", "afl-clang-fast++")
        .current_dir(openssl_dir.clone())
        .status()
        .expect("config openssl失败");

    // 执行make编译openssl
    Command::new("make")
        .current_dir(openssl_dir.clone())
        .status()
        .expect("openssl库编译失败");

    // 构建模糊测试程序
    Command::new("afl-clang-fast++")
        .env("AFL_USE_ASAN", "1")
        .arg("-g")
        .arg("afl_handshake_fuzzer.cc")
        .arg("openssl-1.0.1f/libssl.a")
        .arg("openssl-1.0.1f/libcrypto.a")
        .arg("-std=c++14")
        .arg("-I")
        .arg("openssl-1.0.1f/include/")
        .arg("-lstdc++fs")
        .arg("-ldl")
        .arg("-lstdc++")
        .arg("-o")
        .arg("afl_handshake_fuzzer")
        .current_dir(cwd)
        .status()
        .expect("构建模糊测试程序是失败");
}   