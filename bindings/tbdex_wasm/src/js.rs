use js_sys::{Array, Map, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

fn convert_to_object_recursive(value: &JsValue) -> Result<JsValue, JsValue> {
    if value.is_object() {
        if Array::is_array(value) {
            let array = Array::from(value);
            let new_array = Array::new();

            for elem in array.iter() {
                let converted_elem = convert_to_object_recursive(&elem)?;
                new_array.push(&converted_elem);
            }
            Ok(new_array.into())
        } else if value.is_instance_of::<Map>() {
            let map = Map::from(value.clone());
            let obj = Object::new();

            let entries = map.entries();
            for entry in entries {
                let entry = entry?.dyn_into::<Array>()?;
                let key = entry.get(0);
                let val = entry.get(1);

                let converted_val = convert_to_object_recursive(&val)?;
                Reflect::set(&obj, &key, &converted_val)?;
            }
            Ok(obj.into())
        } else {
            let object: Object = value.clone().dyn_into::<Object>()?;
            let obj = Object::new();
            let keys = Object::keys(&object);

            for key in keys.iter() {
                let val = Reflect::get(&object, &key)?;

                let converted_val = convert_to_object_recursive(&val)?;
                Reflect::set(&obj, &key, &converted_val)?;
            }
            Ok(obj.into())
        }
    } else {
        Ok(value.clone())
    }
}

pub fn convert_to_object(js_value: JsValue) -> Result<JsValue, JsValue> {
    convert_to_object_recursive(&js_value)
}
