use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard,
    },
};

use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde::Serialize;

use crate::{ContextExt, Result, Value, ERR_ARGUMENT, ERR_FORMAT};

lazy_static! {
    static ref GLOBAL_TEMPLATE: Arc<Mutex<Handlebars<'static>>> =
        Arc::new(Mutex::new(Handlebars::new()));
    static ref GLOBAL_TEMPLATE_INITED: AtomicBool = AtomicBool::new(false);
}

fn get_handlebars() -> MutexGuard<'static, Handlebars<'static>> {
    let global = GLOBAL_TEMPLATE.lock().unwrap();
    if !GLOBAL_TEMPLATE_INITED.load(Ordering::Relaxed) {
        GLOBAL_TEMPLATE_INITED.store(true, Ordering::Relaxed);
    }
    global
}

/// 根据内容文本渲染模板
pub fn render_template<C>(template: String, context: &C) -> Result<String>
where
    C: Serialize,
{
    match get_handlebars().render_template(template.as_str(), context) {
        Ok(v) => Ok(v),
        Err(e) => Err(ERR_FORMAT
            .msg_detail("模板渲染失败".to_string())
            .cause(anyhow::Error::new(e))),
    }
}

/// 上下文数据类型
pub enum ContextType {
    /// 模板类型
    TemplateType {
        template: String,
        attrs: Vec<String>,
    },
    /// 值类型
    ValueType(Value),
    /// 调用类型
    InvokerType(Box<dyn Fn(&mut HashMap<String, ContextType>) -> Value>),
}

impl ContextType {
    pub fn get_value(&self) -> &Value {
        if let ContextType::ValueType(v) = self {
            v
        } else {
            panic!("不是ValueType类型");
        }
    }
}

/// 根据模板递归调用子模板、计算类型及上下文进行渲染
pub fn render_template_recursion(
    context: &HashMap<String, ContextType>,
    key: &str,
) -> Result<String> {
    let (root_template, root_attrs) = match context.get(&key.to_string()) {
        Some(v) => match v {
            ContextType::TemplateType { template, attrs } => (template.clone(), attrs),
            _ => return Err(ERR_ARGUMENT.msg_detail(format!("{}不是模板类型", &key))),
        },
        None => return Err(ERR_ARGUMENT.msg_detail(format!("模板定义{}不存在", &key))),
    };
    if root_attrs.is_empty() {
        return Ok(root_template.to_string());
    }
    let mut param = HashMap::<String, Value>::new();
    for item_name in root_attrs {
        match context.get(item_name) {
            Some(child_v) => match child_v {
                ContextType::TemplateType {
                    template: _,
                    attrs: _,
                } => {
                    param.insert(
                        item_name.to_string(),
                        Value::String(render_template_recursion(context, item_name).unwrap()),
                    );
                }
                ContextType::ValueType(v) => {
                    param.insert(item_name.to_string(), v.clone());
                }
                ContextType::InvokerType(it) => {
                    let context = unsafe {
                        &mut *(context as *const HashMap<String, ContextType>
                            as *mut HashMap<String, ContextType>)
                    };
                    param.insert(item_name.to_string(), it.as_ref()(context));
                }
            },
            None => return Err(ERR_ARGUMENT.msg_detail(format!("模板定义{}不存在", item_name))),
        };
    }
    render_template(root_template, &param)
}

/// 键为字符类型且值为ContextType的Map工具操作类，主要用于模板生成
pub trait TemplateContextExt {
    /// 插入模板类型
    fn insert_template(&mut self, key: &str, template: &str, attrs: Vec<&str>);
    /// 插入可调用类型
    fn insert_invoker(
        &mut self,
        key: &str,
        invoker: Box<dyn Fn(&mut HashMap<String, ContextType>) -> Value>,
    );
}
impl ContextExt for HashMap<String, ContextType> {
    fn get_value(&mut self, key: &str) -> Option<&Value> {
        self.get(key).map(|x| x.get_value())
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), ContextType::ValueType(value));
    }
}
impl TemplateContextExt for HashMap<String, ContextType> {
    fn insert_template(&mut self, key: &str, template: &str, attrs: Vec<&str>) {
        self.insert(
            key.to_string(),
            ContextType::TemplateType {
                template: template.to_string(),
                attrs: attrs.iter().map(|x| x.to_string()).collect(),
            },
        );
    }

    fn insert_invoker(
        &mut self,
        key: &str,
        invoker: Box<dyn Fn(&mut HashMap<String, ContextType>) -> Value>,
    ) {
        self.insert(key.to_string(), ContextType::InvokerType(invoker));
    }
}
/// 可代替HashMap<String, ContextType>操作的工具
pub type TemplateContext = HashMap<String, ContextType>;
