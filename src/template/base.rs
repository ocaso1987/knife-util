use std::{
    cell::RefCell,
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard,
    },
};

use handlebars::Handlebars;
pub use lazy_static::lazy_static;

use crate::Value;

use super::helper::{place::place_helper, sql_page::SqlPageHelper};

lazy_static! {
    pub(crate) static ref GLOBAL_TEMPLATE: Arc<Mutex<Handlebars<'static>>> =
        Arc::new(Mutex::new(Handlebars::new()));
    pub(crate) static ref GLOBAL_TEMPLATE_INITED: AtomicBool = AtomicBool::new(false);
}

thread_local! (
    pub(crate) static PLACE_CONTEXT: RefCell<BTreeMap<String,Value>>  = RefCell::new(BTreeMap::new())
);

pub(crate) fn get_handlebars() -> MutexGuard<'static, Handlebars<'static>> {
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
    global.register_helper("sql_page", Box::new(SqlPageHelper {}));
}
