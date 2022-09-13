use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, Renderable,
};

/// 生成占位符，可用于SQL但不具限于SQL拼装场景
#[derive(Clone)]
pub(crate) struct SqlPageHelper {}

impl HelperDef for SqlPageHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let context_data = ctx.data().as_object().unwrap();
        let is_count_sql = context_data.contains_key("_sql_type")
            && context_data.get("_sql_type").unwrap().as_str().unwrap() == "page_count";
        let label;
        let count_label;
        if h.is_block() {
            label = h
                .template()
                .map(|t| t.renders(r, ctx, rc).unwrap().to_string())
                .unwrap_or("".to_string());
            count_label = h
                .hash_get("count_label")
                .map(|x| x.value().as_str().unwrap().to_string())
                .unwrap_or("".to_string());
        } else {
            label = h
                .hash_get("label")
                .map(|x| x.value().as_str().unwrap().to_string())
                .unwrap_or("".to_string());
            count_label = h
                .hash_get("count_label")
                .map(|x| x.value().as_str().unwrap().to_string())
                .unwrap_or("count(*)".to_string());
        }

        if is_count_sql {
            out.write(count_label.as_str()).unwrap();
        } else {
            out.write(label.as_str()).unwrap();
        }
        Ok(())
    }
}
