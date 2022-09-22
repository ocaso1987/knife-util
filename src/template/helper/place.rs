use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

use crate::template::base::PLACE_CONTEXT;

/// 生成占位符，可用于SQL但不具限于SQL拼装场景
pub(crate) fn place_helper(
    h: &Helper,
    _hb: &Handlebars,
    _c: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> std::result::Result<(), RenderError> {
    PLACE_CONTEXT.with(|ctx| {
        let mut map = ctx.borrow_mut();
        let value = h
            .param(0)
            .ok_or_else(|| RenderError::new("参数不能为空."))
            .unwrap();
        let name = h.param(1);
        if let Some(v) = name {
            let key = v.render();
            out.write(key.as_str()).unwrap();
            map.insert(key, value.relative_path().unwrap().clone());
        } else {
            let pos = map.len();
            let key = format!("${}", pos + 1);
            out.write(key.as_str()).unwrap();
            map.insert(key, value.relative_path().unwrap().clone());
        }
    });
    Ok(())
}
