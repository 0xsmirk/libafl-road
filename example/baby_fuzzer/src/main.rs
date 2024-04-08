// #cfg[(windows)]是一个条件编译的属性。当且仅当当前目标系统是Windows时，编译器才会编译被该属性标注的代码
#[cfg(windows)]
use std::ptr::write_volatile;
use std::{path::PathBuf, ptr::write};

// 使用tui图形化展示
#[cfg(feature = "tui")]
use libafl::monitors::tui::{ui::TuiUI, TuiMonitor};

// 使用终端的简单监控组件
#[cfg(not(feature = "tui"))]
use libafl::monitors::SimpleMonitor;

// 调用LibAFL组件
use libafl::{
    corpus::{InMemoryCorpus, OnDiskCorpus},
    events::SimpleEventManager,
    executors::{inprocess::InProcessExecutor, ExitKind},
    feedbacks::{CrashFeedback, MaxMapFeedback},
    fuzzer::{Fuzzer, StdFuzzer},
    generators::RandPrintablesGenerator,
    inputs::{BytesInput, HasTargetBytes},
    mutators::scheduled::{havoc_mutations, StdScheduledMutator},
    observers::StdMapObserver,
    schedulers::QueueScheduler,
    stages::mutational::StdMutationalStage,
    state::StdState,
};

// LibAFL bolts组件
use libafl_bolts::{current_nanos, rands::StdRand, tuples::tuple_list, AsSlice};

// 由于缺乏仪器，覆盖范围图有明确的分配
// 因为测试的目标为当前中的一个函数，使用如下进行判别
static mut SIGNALS: [u8; 16] = [0; 16]; // 定义数组
static mut SIGNALS_PTR: *mut u8 = unsafe { SIGNALS.as_mut_ptr() };

// 为信号图分配信号
fn signals_set(idx: usize) {
    unsafe { write(SIGNALS_PTR.add(idx), 1) };
}

// 模糊的目标忽略检测:告诉Rust的Clippy工具(一个静态代码分析工具),在对特定代码进行检测时忽略某些lint警告的
#[allow(clippy::similar_names, clippy::manual_assert)]
fn main() {
    println!("Baby fuzzer demo running!!!!");

    // 我们想要模糊的闭包
    // 它采用缓冲区作为输入，如果它以"abc"开头，则会出现panic(崩溃)
    // The closure that we want to fuzz
    let mut harness = |input: &BytesInput| {
        let target = input.target_bytes();
        let buf = target.as_slice();
        signals_set(0);
        if !buf.is_empty() && buf[0] == b'a' {
            signals_set(1);
            if buf.len() > 1 && buf[1] == b'b' {
                signals_set(2);
                if buf.len() > 2 && buf[2] == b'c' {
                    #[cfg(unix)]
                    panic!("Artificial bug triggered =)");

                    // panic!() raises a STATUS_STACK_BUFFER_OVERRUN exception which cannot be caught by the exception handler.
                    // Here we make it raise STATUS_ACCESS_VIOLATION instead.
                    // Extending the windows exception handler is a TODO. Maybe we can refer to what winafl code does.
                    // https://github.com/googleprojectzero/winafl/blob/ea5f6b85572980bb2cf636910f622f36906940aa/winafl.c#L728
                    #[cfg(windows)]
                    unsafe {
                        write_volatile(0 as *mut u32, 0);
                    }
                }
            }
        }
        // ExitKind 用于通知模糊器有关线束的退出状态
        ExitKind::Ok
    };

    // To test the panic:
    #[cfg(feature = "panic")]
    let input = BytesInput::new(Vec::from("abc"));
    #[cfg(feature = "panic")]
    harness(&input);

    // Create an observation channel using the signals map
    // [创建观察者]使用信号图创建观察通道
    let observer = unsafe { StdMapObserver::from_mut_ptr("signals", SIGNALS_PTR, SIGNALS.len()) };

    // 反馈者1:对输入内容的趣味性进行评价的反馈(将观察者观测到的信号量给反馈者)
    let mut feedback = MaxMapFeedback::new(&observer);

    // 反馈者2:主要追踪crash的情况
    // 选择输入是否为解决方案的反馈(确定崩溃的情况)
    let mut objective = CrashFeedback::new();

    // create a State from scratch
    // 创建State将随机的输入、代码覆盖率、语料库、观察者、反馈者组合起来
    let mut state = StdState::new(
        // RNG
        StdRand::with_seed(current_nanos()),
        // Corpus that will be evolved, we keep it in memory for performance
        InMemoryCorpus::new(),
        // Corpus in which we store solutions (crashes in this example),
        // on disk so the user can get them after stopping the fuzzer
        OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
        // States of the feedbacks.
        // The feedbacks can report the data that should persist in the State.
        &mut feedback,
        // Same for objective feedbacks
        &mut objective,
    )
    .unwrap();

    // The Monitor trait define how the fuzzer stats are displayed to the user
    // 设置不同展示的Monitor(监控)
    #[cfg(not(feature = "tui"))]
    let mon = SimpleMonitor::new(|s| println!("{s}"));
    #[cfg(feature = "tui")]
    let ui = TuiUI::with_version(String::from("Baby Fuzzer"), String::from("0.0.1"), false);
    #[cfg(feature = "tui")]
    let mon = TuiMonitor::new(ui);

    // 设置事件管理器
    // 事件管理器处理模糊测试循环期间生成的各种事件
    // 例如语料库中添加新项目的通知
    let mut mgr = SimpleEventManager::new(mon);

    // A queue policy to get testcasess from the corpus
    // 从语料库中获取测试数据的队列策略
    let scheduler = QueueScheduler::new();

    // A fuzzer with feedbacks and a corpus scheduler
    // 带有反馈的模糊器和语料库调度器
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    // Create the executor for an in-process function with just one observer
    // 为进程中函数创建执行器，只需一个观察者
    let mut executor = InProcessExecutor::new(
        &mut harness,           // 添加待模糊目标
        tuple_list!(observer),  // 添加观察者
        &mut fuzzer,            // 添加fuzzer(语料库调度策略、反馈者、观察者)
        &mut state,             // 添加State
        &mut mgr,               // 添加事件管理器
    )
    .expect("Failed to create the Executor");

    // =============自定义输入生成器================
    // Generator of printable bytearrays of max size 32
    // 生成最大大小为 32 的可打印字节数组
    let mut generator = RandPrintablesGenerator::new(32);

    // Generate 8 initial inputs
    // 生成 8 个初始输入
    state
        .generate_initial_inputs(&mut fuzzer, &mut executor, &mut generator, &mut mgr, 8)
        .expect("Failed to generate the initial corpus");
    // =============自定义输入生成器================

    // Setup a mutational stage with a basic bytes mutator
    // [突变器]使用基本字节突变器设置突变阶段
    let mutator = StdScheduledMutator::new(havoc_mutations());
    let mut stages = tuple_list!(StdMutationalStage::new(mutator));

    // 启动fuzz循环开始
    fuzzer
        .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
        .expect("Error in the fuzzing loop");
}
