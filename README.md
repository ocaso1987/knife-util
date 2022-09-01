# knife-util
Knife框架基础工具包

[![Crates.io](https://img.shields.io/crates/v/knife-util)](https://crates.io/crates/knife-util)
[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/ocaso1987/knife-util/knife-util)](https://github.com/ocaso1987/knife-util)
[![docs.rs](https://img.shields.io/docsrs/knife-util)](https://docs.rs/knife-util)

## 说明
本项目用于为knife-framework微服务框架定制的util工具集合，这些工具包可以独立于knife-framework框架之外进行使用。

## 包含以下工具包:
### any工具
* **AnyValue:** 支持存入数据并以对象方式取出，也可以多次使用其指针，可以用于代替Box<dyn Any>类型，优点是可以取出原始类型并忽略生命周期限制。
* **AnyRef:** 支持绑定指针并使用其指针，可以用于代替&，优点是可以忽略生命周期限制。

### bean工具
统一封装serde_json::Value、serde_yaml::Value、toml::Value及Bson格式并实现相互转换
* **PointerExt:** 通过/a/b/c格式快速定位到子级内容
* **MergeExt:** 实现数据对象的合并操作
* **BsonConvertExt:** 转换为Bson格式

### error工具
统一异常封装，以支持全局异常错误码处理。

### context工具
* **ContextExt:** 统一提供对上下文对象存入取出基本类型数据
* **AnyContextExt:** 统一提供对上下文对象存入任意类型数据
### number工具
提供数值类型的数据转换功能

### future工具
* **FutureHandler:** 用来代替dyn FnOnce(E) -> (dyn Future<Output = R> + Send + 'static) + Send + 'static)的工具
* **FutureObj:** 用来代替dyn Future<Output = R> + Send + 'static的工具
### string工具
* **StringExt:** 提供字段基本转换及正则匹配操作

### template工具
* **render_simple_template:** 根据字符内容渲染handlerbars模板
* **render_template:** 根据字符内容渲染handlerbars模板，支持采用占位符替换某参数，并返回占位符指定参数组成的集合
* **render_template_recursion:** 根据模板递归调用子模板、计算类型及上下文渲染模板

### vec工具
* **VecExt:** 对vec对象进行数据处理及转换操作
