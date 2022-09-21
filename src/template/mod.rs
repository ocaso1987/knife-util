//! 模板工具类
//!
//! 可以用于生成html或者SQL等文本
mod base;
mod context;
mod render;
mod tests;
mod helper;

pub use context::{ContextType, TemplateContextExt};
pub use render::{
    render_simple_template, render_sql_template, render_template, render_template_recursion,
};
