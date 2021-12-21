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

#[wasm_bindgen]
pub struct Person{
     name: String,
     age: i32,
}

#[wasm_bindgen]
impl Person {
     #[wasm_bindgen(getter)]
     pub fn name(&self) -> String{
          format!("{}",self.name)
     }

     #[wasm_bindgen(setter)]
     pub fn set_name(&mut self,name:&str) {
          self.name = format!("{}",name);
     }

     #[wasm_bindgen(constructor)]
     pub fn new() -> Person{
          Person{name:String::from("new constructor"),age:18}
     }
     pub fn get_name(&self) -> String {
               format!("{}",self.name)
     }
     pub fn get_age(&self) ->i32{
          self.age
     }
}

//定义一个虚假的构造函数
#[wasm_bindgen]
pub fn new_person(name:&str,age:i32)->Person{
     Person{name:name.to_string(),age:age}
}