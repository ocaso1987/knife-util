use std::collections::HashMap;

use crate::{context::ContextTrait, Ok, Result, Value};

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

impl std::fmt::Debug for ContextType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TemplateType { template, attrs } => f
                .debug_struct("TemplateType")
                .field("template", template)
                .field("attrs", attrs)
                .finish(),
            Self::ValueType(arg0) => f.debug_tuple("ValueType").field(arg0).finish(),
            Self::InvokerType(_) => f.debug_tuple("InvokerType").finish(),
        }
    }
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

/// 键为字符类型且值为ContextType的Map工具操作类，主要用于模板生成
pub trait TemplateContextExt {
    /// 插入模板类型
    fn insert_template(&mut self, key: &str, template: &str, attrs: Vec<String>);
    /// 插入可调用类型
    fn insert_invoker(
        &mut self,
        key: &str,
        invoker: Box<dyn Fn(&mut HashMap<String, ContextType>) -> Value>,
    );
}

impl ContextTrait for HashMap<String, ContextType> {
    type Context = ContextType;
    fn get_value(&self, key: &str) -> Result<Option<&Value>> {
        Ok(self.get(key).map(|x| x.get_value()))
    }

    fn insert_value(&mut self, key: &str, value: Value) -> Result<()> {
        self.insert(key.to_string(), ContextType::ValueType(value));
        Ok(())
    }
}

impl TemplateContextExt for HashMap<String, ContextType> {
    fn insert_template(&mut self, key: &str, template: &str, attrs: Vec<String>) {
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
