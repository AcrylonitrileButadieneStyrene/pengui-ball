mod input;
mod output;

pub use input::InputToken;
pub use output::OutputToken;

pub enum Token {
    Input(InputToken),
    Output(OutputToken),
}
