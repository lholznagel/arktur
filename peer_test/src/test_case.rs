use futures_cpupool::{CpuFuture, CpuPool};

pub trait TestCase {
    fn description() -> String;

    fn name() -> String;

    fn execute(cpu_pool: &CpuPool) -> CpuFuture<bool, ()>;
}