use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn main(comando:String)-> String{
    let mut c=String::from("");
    if comando == "help"{
        c=help();
    }else if comando == "cat flag.txt"{
        c=cat();
    }else if comando == "ls"{
        c=ls();
    }else{
        c=String::from("N0t f0und l0l");
    }
    return c;
}
fn help()->String{
    return String::from("Comandos Disponibles:<br>1: ls<br>2: cd<br>3: mkdir");
}
fn ls()->String{
    return String::from("flag.txt README.md");
}
fn cat()->String{
    return String::from("CSU{Noc_n0c_n0t_broma}");
}
