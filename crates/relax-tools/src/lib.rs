pub mod builtin {
    pub mod read_file;
    pub mod shell;
    pub mod update_plan;
    pub mod write_file;
}

mod registry;
mod tool;

pub use registry::ToolRegistry;
pub use tool::{Tool, ToolError, ToolResult, ToolSchema};
