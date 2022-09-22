use std::collections::{BTreeMap, HashMap};

use crate::{
    bean::FromValueTrait,
    context::ContextTrait,
    error::{ERR_ARGUMENT, ERR_FORMAT},
    types::StringExt,
    Result, Value, OK,
};

use super::{
    base::{get_handlebars, PLACE_CONTEXT},
    context::{ContextType, TemplateContextExt},
};

/// 根据内容文本渲染模板
pub fn render_simple_template(template: String, value: &Value) -> Result<String> {
    let param = value.as_object()?;
    let mut ctx = handlebars::Context::null();
    if param.contains_key("_root") && param.len() == 1 {
        *ctx.data_mut() = serde_json::Value::from_value(param.get("_root").unwrap()).unwrap()
    } else {
        *ctx.data_mut() = serde_json::Value::from_value(value).unwrap()
    };
    match get_handlebars().render_template_with_context(template.as_str(), &ctx) {
        Ok(v) => OK(v),
        Err(e) => Err(ERR_FORMAT.msg_detail("模板渲染失败").cause(e)),
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
                map.insert_value(k.as_str(), v.clone()).unwrap();
                attrs.push(k.to_string());
            }
        }
        v => {
            map.insert_value("_root", v.clone()).unwrap();
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

#[allow(clippy::cast_ref_to_mut)]
fn render_template_recursion_inner(
    context: &HashMap<String, ContextType>,
    key: &str,
) -> Result<(String, BTreeMap<String, Value>)> {
    let (root_template, root_attrs) = match context.get(&key.to_string()) {
        Some(v) => match v {
            ContextType::TemplateType { template, attrs } => (template.clone(), attrs),
            _ => {
                return Err(ERR_ARGUMENT.msg_detail(format!("{}不是ContextType类型", &key).as_str()))
            }
        },
        None => return Err(ERR_ARGUMENT.msg_detail(format!("模板定义{}不存在", &key).as_str())),
    };
    let context_mut = unsafe {
        &mut *(context as *const HashMap<String, ContextType> as *mut HashMap<String, ContextType>)
    };
    let param = &mut BTreeMap::<String, Value>::new();
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
                            render_template_recursion_inner(context, item_name)?.0,
                        )?;
                    }
                    ContextType::ValueType(v) => {
                        param.insert_value(item_name, v.clone())?;
                    }
                    ContextType::InvokerType(it) => {
                        param.insert_value(item_name, it.as_ref()(context_mut))?;
                    }
                },
                None => {
                    return Err(
                        ERR_ARGUMENT.msg_detail(format!("模板定义{}不存在", item_name).as_str())
                    )
                }
            };
        }
    }
    match context_mut.get_value("_root") {
        Ok(v) => match v {
            Some(v2) => param.insert("_root".to_string(), v2.clone()),
            None => None,
        },
        Err(e) => return Err(e),
    };

    let res = render_simple_template(root_template, &Value::Object(param.clone()))?;
    let mut res_map = BTreeMap::new();
    PLACE_CONTEXT.with(|ctx| {
        for v in ctx.borrow().iter() {
            let value = param.get(v.1).unwrap().clone();
            res_map.insert(v.0.to_string(), value);
        }
    });
    OK((res, res_map))
}
