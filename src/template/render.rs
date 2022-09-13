use std::collections::{BTreeMap, HashMap};

use crate::{
    context::ContextExt,
    error::{ERR_ARGUMENT, ERR_FORMAT},
    types::StringExt,
    value::Value,
    Result,
};

use super::{
    base::{get_handlebars, PLACE_CONTEXT},
    context::{ContextType, TemplateContextExt},
};

/// 根据内容文本渲染模板
pub fn render_simple_template(template: String, value: &Value) -> Result<String> {
    let param = value.as_object().unwrap();
    let ctx;
    if param.contains_key("_root") && param.len() == 1 {
        ctx = handlebars::Context::wraps(param.get("_root")).unwrap();
    } else {
        ctx = handlebars::Context::wraps(param).unwrap();
    }
    match get_handlebars().render_template_with_context(template.as_str(), &ctx) {
        Ok(v) => Ok(v),
        Err(e) => Err(ERR_FORMAT.msg_detail("模板渲染失败".to_string()).cause(e)),
    }
}

/// 根据SQL文本渲染模板，返回的结果包括占位符及变量
pub fn render_sql_template(template: String, param: &Value) -> Result<(String, Vec<Value>)> {
    render_template(template, param).map(|(a, b)| (a.compact(), Vec::from_iter(b.into_values())))
}

/// 根据内容文本渲染模板，并返回占位符集合
pub fn render_template(
    template: String,
    param: &Value,
) -> Result<(String, BTreeMap<String, Value>)> {
    let mut map = HashMap::<String, ContextType>::new();
    let key = "$template";
    let mut attrs = vec![];
    match param {
        Value::Object(obj) => {
            for (k, v) in obj {
                map.insert_value(k.as_str(), v.clone());
                attrs.push(k.to_string());
            }
        }
        v => {
            map.insert_value("_root", v.clone());
        }
    }
    map.insert_template(key, template.as_str(), attrs);
    render_template_recursion(&map, key)
}

/// 根据模板递归调用子模板、计算类型及上下文进行渲染，支持返回占用类型的参数
pub fn render_template_recursion(
    context: &HashMap<String, ContextType>,
    key: &str,
) -> Result<(String, BTreeMap<String, Value>)> {
    PLACE_CONTEXT.with(|ctx| {
        ctx.borrow_mut().clear();
        let res = render_template_recursion_inner(context, key);
        ctx.borrow_mut().clear();
        res
    })
}

fn render_template_recursion_inner(
    context: &HashMap<String, ContextType>,
    key: &str,
) -> Result<(String, BTreeMap<String, Value>)> {
    let (root_template, root_attrs) = match context.get(&key.to_string()) {
        Some(v) => match v {
            ContextType::TemplateType { template, attrs } => (template.clone(), attrs),
            _ => return Err(ERR_ARGUMENT.msg_detail(format!("{}不是ContextType类型", &key))),
        },
        None => return Err(ERR_ARGUMENT.msg_detail(format!("模板定义{}不存在", &key))),
    };
    let context_mut = unsafe {
        &mut *(context as *const HashMap<String, ContextType> as *mut HashMap<String, ContextType>)
    };
    let mut param = BTreeMap::<String, Value>::new();
    if !root_attrs.is_empty() {
        for item_name in root_attrs {
            match context.get(item_name) {
                Some(child_v) => match child_v {
                    ContextType::TemplateType {
                        template: _,
                        attrs: _,
                    } => {
                        param.insert_string(
                            item_name.as_str(),
                            render_template_recursion_inner(context, item_name)
                                .unwrap()
                                .0,
                        );
                    }
                    ContextType::ValueType(v) => {
                        param.insert_value(item_name, v.clone());
                    }
                    ContextType::InvokerType(it) => {
                        param.insert_value(item_name, it.as_ref()(context_mut));
                    }
                },
                None => return Err(ERR_ARGUMENT.msg_detail(format!("模板定义{}不存在", item_name))),
            };
        }
    }
    if context.contains_key("_root") {
        param.insert(
            "_root".to_string(),
            context_mut.get_value("_root").unwrap().clone(),
        );
    }
    let res = render_simple_template(root_template, &Value::Object(param));
    res.map(|x| (x, PLACE_CONTEXT.with(|ctx| ctx.borrow().clone())))
}
