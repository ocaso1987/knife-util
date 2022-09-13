//! 模板工具类
//!
//! 可以用于生成html或者SQL等文本
pub(crate) mod base;

pub(crate) mod helper {
    pub(crate) mod place;
    pub(crate) mod sql_page;
}

pub(crate) mod context;
pub(crate) mod render;

pub(crate) mod tests;

pub use context::{ContextType, TemplateContextExt};
pub use render::{
    render_simple_template, render_sql_template, render_template, render_template_recursion,
};
