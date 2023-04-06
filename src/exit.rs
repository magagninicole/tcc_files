#[derive(Debug, Clone, Copy)]
pub enum ExitCode {
    Success,
    Failed,
    Reset,
}

pub trait Exit {
    fn new(addr: usize, reason: ExitCode) -> Self;
    fn exit(&self, reason: ExitCode) -> !;
    fn exit_success(&self) -> !;
    fn exit_failure(&self) -> !;
}
