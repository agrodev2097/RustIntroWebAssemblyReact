use std::cell::Ref;

use js_sys::Reflect;
use tetris::Tetris;
use wasm_bindgen::JsValue;
use wasm_react::{c, h, Component, hooks::use_state, export_components};

mod shape;
mod tetris;

pub struct App{
    height: u32,
    width: u32,
}

impl TryFrom <JsValue> for App {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(App {
            width: Reflect::get(&value, &"width".into())?
            .as_f64()
            .unwrap_or(10.0) as u32,
            height: Reflect::get(&value, &"height".into())?
            .as_f64()
            .unwrap_or(30.0) as u32,
        })
    }
}

impl Component for App {
    fn render(&self) -> wasm_react:: VNode {
        let tetris = use_state(|| Tetris::new(self.width, self.height));

        h!(div).build(c![
            ..tetris.value().iter_positions().map(|pos| {
                let typ = tetris.value().get(pos);
                
                h!(div).build(c![typ.unwrap_or_default()])
            })
        ])        
    }
}

export_components! { App }

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

