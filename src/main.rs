use anyhow::Result;

pub mod util;

macro_rules! tasks {
    ($fn_name:ident, {$($task:ident),* $(,)?}) => {
        pub mod tasks {
            $(pub mod $task;)*
        }
        fn $fn_name() -> Result<()> {
            $(tasks::$task::run()?;)*
            Ok(())
        }
    };
}

// uncomment implemented tasks :D
tasks!(run_tasks, {
    // aoc_01_1,
    // aoc_01_2,
    // aoc_02_1,
    // aoc_02_2,
    // aoc_03_1,
    // aoc_03_2,
    // aoc_04_1,
    // aoc_04_2,
    // aoc_05_1,
    // aoc_05_2,
    // aoc_06_1,
    // aoc_06_2,
    // aoc_07_1,
    // aoc_07_2,
    // aoc_08_1,
    // aoc_08_2,
    // aoc_09_1,
    // aoc_09_2,
    // aoc_10_1,
    // aoc_10_2,
    // aoc_11_1,
    // aoc_11_2,
    // aoc_12_1,
    // aoc_12_2,
    // aoc_13_1,
    // aoc_13_2,
    // aoc_14_1,
    // aoc_14_2,
    // aoc_15_1,
    // aoc_15_2,
    // aoc_16_1,
    // aoc_16_2,
    // aoc_17_1,
    // aoc_17_2,
    // aoc_18_1,
    // aoc_18_2,
    // aoc_19_1,
    // aoc_19_2,
    // aoc_20_1,
    // aoc_20_2,
    // aoc_21_1,
    // aoc_21_2,
    // aoc_22_1,
    // aoc_22_2,
    // aoc_23_1,
    // aoc_23_2,
    // aoc_24_1,
    // aoc_24_2,
    // aoc_25_1,
    // aoc_25_2,
});

fn main() -> Result<()> {
    run_tasks()
}
