# openssl Heartbleed

## 0x01.简介

由于`AFL++`可对`openssl`的`heartbleed`进行模糊测试，这里使用`LibAFL`进行模糊测试

## 0x02.运行

先运行当前目录的`env_build.sh`安装`openssl 1.0.1f`源码及依赖环境
```shell
$ chmod +x env_build.sh && ./env_build.sh
```

再运行如下命令编译`openssl heartbleed`对模糊测试程序

```shell
smile@pt:~/libAFL/example$ cargo build
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
    Blocking waiting for file lock on build directory
   Compiling openssl_heartbleed v0.1.0 (/home/smile/libAFL/example/openssl_heartbleed)
warning: `openssl_heartbleed` (bin "openssl_heartbleed") generated 1 warning (run `cargo fix --bin "openssl_heartbleed"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 2m 04s
```

一段时间即可出现`crash`

```shell
smile@pt:~/libAFL/example/openssl_heartbleed$ ../target/debug/openssl_heartbleed
openssl heartbleed libafl fuzzing!!
[Stats #0] run time: 0h-0m-0s, clients: 1, corpus: 0, objectives: 0, executions: 0, exec/sec: 0.000
[Testcase #0] run time: 0h-0m-0s, clients: 1, corpus: 1, objectives: 0, executions: 1, exec/sec: 0.000
We imported 1 inputs from disk.
[Stats #0] run time: 0h-0m-0s, clients: 1, corpus: 1, objectives: 0, executions: 1, exec/sec: 0.000
[Testcase #0] run time: 0h-0m-0s, clients: 1, corpus: 2, objectives: 0, executions: 3, exec/sec: 0.000
[Stats #0] run time: 0h-0m-1s, clients: 1, corpus: 2, objectives: 0, executions: 3, exec/sec: 1.722
[Testcase #0] run time: 0h-0m-1s, clients: 1, corpus: 3, objectives: 0, executions: 182, exec/sec: 104.3
[Stats #0] run time: 0h-0m-1s, clients: 1, corpus: 3, objectives: 0, executions: 182, exec/sec: 95.90
[Testcase #0] run time: 0h-0m-1s, clients: 1, corpus: 4, objectives: 0, executions: 198, exec/sec: 104.2
[Stats #0] run time: 0h-0m-3s, clients: 1, corpus: 4, objectives: 0, executions: 198, exec/sec: 60.88
[Testcase #0] run time: 0h-0m-3s, clients: 1, corpus: 5, objectives: 0, executions: 338, exec/sec: 103.9
[Stats #0] run time: 0h-0m-4s, clients: 1, corpus: 5, objectives: 0, executions: 338, exec/sec: 81.10
[Testcase #0] run time: 0h-0m-4s, clients: 1, corpus: 6, objectives: 0, executions: 433, exec/sec: 103.8
............
[Objective #0] run time: 0h-2m-6s, clients: 1, corpus: 11, objectives: 1, executions: 13005, exec/sec: 102.4
[Stats #0] run time: 0h-2m-18s, clients: 1, corpus: 11, objectives: 1, executions: 14624, exec/sec: 105.9
[Stats #0] run time: 0h-2m-21s, clients: 1, corpus: 11, objectives: 1, executions: 14624, exec/sec: 103.4
[Testcase #0] run time: 0h-2m-21s, clients: 1, corpus: 12, objectives: 1, executions: 14977, exec/sec: 105.9
[Stats #0] run time: 0h-2m-33s, clients: 1, corpus: 12, objectives: 1, executions: 16281, exec/sec: 106.0
[Stats #0] run time: 0h-2m-41s, clients: 1, corpus: 12, objectives: 1, executions: 16281, exec/sec: 101.0
[Testcase #0] run time: 0h-2m-41s, clients: 1, corpus: 13, objectives: 1, executions: 17092, exec/sec: 106.1
```
