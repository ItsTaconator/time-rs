use clap::Parser;

#[derive(Clone, Parser)]
pub struct Arguments {
    /// Allow stdout to print to terminal
    #[arg(long, short = 'o', default_value_t = false)]
    pub stdout: bool,
    /// Times command should be ran
    #[arg(long, short, default_value_t = 1)]
    pub times: usize,
    /// Command to run
    #[arg(index = 1, trailing_var_arg = true)]
    pub command: Vec<String>,
}
