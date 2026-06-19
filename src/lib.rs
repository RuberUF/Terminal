use wasm_bindgen::prelude::*;
use std::collections::HashMap;
#[wasm_bindgen]
pub fn main(comando:String)-> String{
    let partes: Vec<&str>=comando.split_whitespace().collect();
    let mut archivos: HashMap<&str,&str>=HashMap::new();
    archivos.insert("flag.txt","FLAG{nocnoc_noc}");
    archivos.insert("readme.txt","welcome :3");
    let mut ls: Vec<&str>=Vec::new();
    let c = if partes.len()== 0{
        String::from("")
    }
    else{
        if partes[0] == "help"{
            help()
        }else if partes[0] == "cat"{
            match archivos.get(partes[1]){
                Some(contenido)=> String::from(*contenido),
                None => String::from("Error"),  
            }
        }else if partes[0] == "ls"{
            for llaves in archivos.keys() {
                ls.push(*llaves);
            }
            ls.join(" ")
        }else{
            String::from("N0t f0und l0l")
        }
    };
    return c;
}
fn help()->String{
    return String::from("comandos disponibles<br>ls<br>cat")
}
