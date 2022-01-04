extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
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

#[wasm_bindgen(start)]
pub fn run() ->Result<(), JsValue>  {
     //获取一个window对象
     let window = web_sys::window().expect("get window error");
     //获取一个document 对象
     let document = window.document().expect("get document error");
     //获取一个body 节点
     let body = document.body().expect("get body error");
    //创建一个button标签
      let button= document.create_element("button")?;
     button.set_inner_html("点击");
     //追加到body里面 
     body.append_child(&button)?;


     let mut clicks = 0;
    let a = Closure::wrap(Box::new(move || {
        clicks += 1;
        button.set_inner_html(&clicks.to_string());
    }) as Box<dyn FnMut()>);
     document
     .get_element_by_id("bt")
     .expect("找不到该id")
     .dyn_ref::<HtmlElement>()
     .expect("转化HtmlElement")
     .set_onclick(Some(a.as_ref().unchecked_ref()));
     a.forget();

     Ok(())
}