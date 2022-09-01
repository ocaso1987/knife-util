use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard,
    },
};

use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use lazy_static::lazy_static;

use crate::{
    value::value::Value, ContextExt, Result, StringExt, ValueConvertExt, ERR_ARGUMENT, ERR_FORMAT,
};

lazy_static! {
    static ref GLOBAL_TEMPLATE: Arc<Mutex<Handlebars<'static>>> =
        Arc::new(Mutex::new(Handlebars::new()));
    static ref GLOBAL_TEMPLATE_INITED: AtomicBool = AtomicBool::new(false);
}
thread_local! (
    static PLACE_CONTEXT: RefCell<BTreeMap<String, Value>> = RefCell::new(BTreeMap::new())
);

fn get_handlebars() -> MutexGuard<'static, Handlebars<'static>> {
    let mut global = GLOBAL_TEMPLATE.lock().unwrap();
    if !GLOBAL_TEMPLATE_INITED.load(Ordering::Relaxed) {
        init(&mut global);
        GLOBAL_TEMPLATE_INITED.store(true, Ordering::Relaxed);
    }
    global
}

/// 模板引擎初始化
fn init(global: &mut MutexGuard<Handlebars>) {
    global.register_helper("$", Box::new(place_helper));
}

/// 生成占位符，可用于SQL但不具限于SQL拼装场景
fn place_helper(
    h: &Helper,
    _hb: &Handlebars,
    _c: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> std::result::Result<(), RenderError> {
    PLACE_CONTEXT.with(|ctx| {
        let mut map = ctx.borrow_mut();
        let value = h.param(0).ok_or(RenderError::new("参数不能为空.")).unwrap();
        let name = h.param(1);
        if name.is_some() {
            let key = format!("{}", name.unwrap().render());
            out.write(key.as_str()).unwrap();
            map.insert(key, value.value().as_value());
        } else {
            let pos = map.len();
            let key = format!("${}", pos + 1);
            out.write(key.as_str()).unwrap();
            map.insert(key, value.value().as_value());
        }
    });
    Ok(())
}

/// 根据内容文本渲染模板
pub fn render_simple_template(template: String, context: &Value) -> Result<String> {
    match get_handlebars().render_template(template.as_str(), context) {
        Ok(v) => Ok(v),
        Err(e) => Err(ERR_FORMAT
            .msg_detail("模板渲染失败".to_string())
            .cause(anyhow::Error::new(e))),
    }
}

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
                map.insert(k.to_string(), ContextType::ValueType(v.clone()));
                attrs.push(k.to_string());
            }
        }
        v => {
            map.insert("_root".to_string(), ContextType::ValueType(v.clone()));
        }
    }
    map.insert_template(key, template.as_str(), attrs);
    render_template_recursion(&map, key)
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

/// 根据模板递归调用子模板、计算类型及上下文进行渲染，支持返回占用类型的参数
pub fn render_template_recursion(
    context: &HashMap<String, ContextType>,
    key: &str,
) -> Result<(String, BTreeMap<String, Value>)> {
    PLACE_CONTEXT.with(|ctx| ctx.borrow_mut().clear());
    let (root_template, root_attrs) = match context.get(&key.to_string()) {
        Some(v) => match v {
            ContextType::TemplateType { template, attrs } => (template.clone(), attrs),
            _ => return Err(ERR_ARGUMENT.msg_detail(format!("{}不是模板类型", &key))),
        },
        None => return Err(ERR_ARGUMENT.msg_detail(format!("模板定义{}不存在", &key))),
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
                            render_template_recursion(context, item_name).unwrap().0,
                        );
                    }
                    ContextType::ValueType(v) => {
                        param.insert(item_name.to_string(), v.clone());
                    }
                    ContextType::InvokerType(it) => {
                        let context_mut = unsafe {
                            &mut *(context as *const HashMap<String, ContextType>
                                as *mut HashMap<String, ContextType>)
                        };
                        param.insert(item_name.to_string(), it.as_ref()(context_mut));
                    }
                },
                None => return Err(ERR_ARGUMENT.msg_detail(format!("模板定义{}不存在", item_name))),
            };
        }
    }
    let res;
    if context.contains_key("_root") {
        let context_mut = unsafe {
            &mut *(context as *const HashMap<String, ContextType>
                as *mut HashMap<String, ContextType>)
        };
        res = render_simple_template(
            root_template,
            context_mut.get_value("_root").as_ref().unwrap(),
        );
    } else {
        res = render_simple_template(root_template, &Value::Object(param));
    }
    res.map(|x| (x, PLACE_CONTEXT.with(|ctx| ctx.take())))
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
impl ContextExt for HashMap<String, ContextType> {
    type Context = ContextType;
    fn get_value(&mut self, key: &str) -> Option<&Value> {
        self.get(key).map(|x| x.get_value())
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), ContextType::ValueType(value));
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
/// 可代替HashMap<String, ContextType>操作的工具
pub type TemplateContext = HashMap<String, ContextType>;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::{
        template::{render_sql_template, render_template, render_template_recursion},
        ContextExt, TemplateContextExt, ValueConvertExt,
    };

    #[test]
    fn test_render_template_recursion() {
        let mut map = HashMap::new();
        map.insert_template(
            "sql",
            r#"
                select * from table where name={{$ data.name}} and address in 
                {{#each data.address}}
                    {{$ city}},
                {{/each}}
            "#,
            vec!["data".to_string()],
        );
        map.insert_value(
            "data",
            json!({
                "name": "zhangshan",
                "age": [13, 14],
                "address": [{
                    "country": "china",
                    "city": "shanghai",
                },{
                    "country": "usa",
                    "city": "newyork",
                }]
            })
            .as_value(),
        );
        let res = render_template_recursion(&map, "sql").unwrap();
        println!("{:?}", res.0);
        println!("{:?}", res.1);
        assert!(res.0.contains("$1"));
        assert!(res.1.contains_key("$1"));
    }

    #[test]
    fn test_render_template1() {
        let res = render_template(
            r#"
                select * from table where name={{$ this}} 
            "#
            .to_string(),
            &json!(["张三"]).as_value(),
        )
        .unwrap();
        println!("{:?}", res.0);
        println!("{:?}", res.1);
        assert!(res.0.contains("$1"));
        assert!(res.1.contains_key("$1"));
    }

    #[test]
    fn test_render_template2() {
        let res = render_template(
            r#"
                select * from table where name in 
                {{#each this}}
                    {{$ this}},
                {{/each}}
            "#
            .to_string(),
            &json!(["张三", "李四"]).as_value(),
        )
        .unwrap();
        println!("{:?}", res.0);
        println!("{:?}", res.1);
        assert!(res.0.contains("$1"));
        assert!(res.1.contains_key("$1"));
    }

    #[test]
    fn test_render_template3() {
        let res = render_template(
            r#"
                select * from table where name={{$ name}} and address in 
                {{#each address}}
                    {{$ city}},
                {{/each}}
            "#
            .to_string(),
            &json!({
                "name": "zhangshan",
                "age": [13, 14],
                "address": [{
                    "country": "china",
                    "city": "shanghai",
                },{
                    "country": "usa",
                    "city": "newyork",
                }]
            })
            .as_value(),
        )
        .unwrap();
        println!("{:?}", res.0);
        println!("{:?}", res.1);
        assert!(res.0.contains("$1"));
        assert!(res.1.contains_key("$1"));
    }

    #[test]
    fn test_render_sql_template() {
        let res = render_sql_template(
            r#"
                select * from table where address in 
                {{#each address}}
                    {{$ city}},
                {{/each}} and name={{$ name}}
            "#
            .to_string(),
            &json!({
                "name": "zhangshan",
                "age": [13, 14],
                "address": [{
                    "country": "china",
                    "city": "shanghai",
                },{
                    "country": "usa",
                    "city": "newyork",
                }]
            })
            .as_value(),
        )
        .unwrap();
        println!("{:?}", res.0);
        println!("{:?}", res.1);
        assert!(res.0.contains("?"));
    }
}
