use clap::Parser;

const DEFAULT_ITERATIONS_COUNT: usize = 5;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cpast", version, author, about, long_about = None)]
#[command(bin_name = "cpast")]
pub(crate) struct CpastCli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Parser)] // requires `derive` feature
#[command(name = "cpast", version, author, about, long_about = None)]
#[command(bin_name = "cpast")]
pub(crate) enum Commands {
    /// Compare two files to find the missing edge case
    Test(TestCliArgs),

    /// Just generate the testcase
    Generate(GeneratorCliArgs),
}

#[derive(clap::Args)]
#[command(author, about, long_about = None)]
pub(crate) struct TestCliArgs {
    /// The correct reference file
    #[arg(short, long, required = true)]
    pub correct_file: Option<String>,

    /// File against which you want to do test
    #[arg(short, long, required = true)]
    pub test_file: Option<String>,

    /// Write Generator LanguageName for generating Tests
    #[arg(short, long, required = true)]
    pub(crate) generator: Option<String>,

    /// Number of times to iterate before finding a correct output
    #[arg(short, long, default_value_t = DEFAULT_ITERATIONS_COUNT)]
    pub(crate) iterations: usize,
}

#[derive(clap::Args)]
#[command(author, about, long_about = None)]
pub(crate) struct GeneratorCliArgs {
    /// Write Generator LanguageName for generating Tests
    pub(crate) generator: Option<String>,
}

impl CpastCli {
    pub fn new() -> Self {
        CpastCli::parse()
    }
}
