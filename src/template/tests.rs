#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::{
        bean::AsValueTrait,
        context::ContextTrait,
        template::{
            context::TemplateContextExt,
            render::{render_sql_template, render_template, render_template_recursion},
        },
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
        map.insert_json(
            "data",
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
            }),
        )
        .unwrap();
        let res = render_template_recursion(&map, "sql").unwrap();
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
            &json!(["张三"]).as_value().unwrap(),
        )
        .unwrap();
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
            &json!(["张三", "李四"]).as_value().unwrap(),
        )
        .unwrap();
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
            .as_value()
            .unwrap(),
        )
        .unwrap();
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
            .as_value()
            .unwrap(),
        )
        .unwrap();
        assert!(res.0.contains("$1"));
    }
}
