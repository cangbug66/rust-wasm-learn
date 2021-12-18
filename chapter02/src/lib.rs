extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
     //导入console.log    js_namespace 指定js包名
     #[wasm_bindgen(js_namespace=console,js_name=log)]
     fn rustlog(s:&str);
}


// macro_rules 用于定义声明宏
macro_rules! dump {
     ( $($ex:expr),*) => {
          $(
               rustlog(format!("{}",$ex).as_str());
          )*
     };
}
#[wasm_bindgen]
pub fn echo(array:JsValue)  {
     let elements: Vec<String> = array.into_serde().unwrap();
     for v in elements {
          dump!(v);
     } 
}