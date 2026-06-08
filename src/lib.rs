use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn main(comando:String)-> String{
    let partes: Vec<&str>=comando.split_whitespace().collect();
    let c = if partes.len()== 0{
        String::from("")
    }
    else{
        if partes[0] == "help"{
            help()
        }else if partes[0] == "cat"{
            if partes.len() > 1{
                cat(partes[1])
            }else {
                String::from("3rr0r, ejemplo(cat file)")
            }
        }else if partes[0] == "ls"{
            ls()
        }else{
            String::from("N0t f0und l0l")
        }
    };
    return c;
}
fn help()->String{
    return String::from("Comandos Disponibles:<br>1: ls<br>2: cd<br>3: mkdir");
}
fn ls()->String{
    return String::from("flag.txt README.md");
}
fn cat(c:&str)->String{
    let d = if c=="flag.txt"{
        String::from("fuck{Noc_n0c_n0t_broma}")
    }else if c == "README.md"{
        String::from("W3lc0m3 :3")
    }else{
        String::from("zzzz")
    };
    return d;
}
