import init,{main} from "../pkg/terminal.js"
await init();
function terminal(){
  document.getElementById("comando").addEventListener("keydown",(evento)=>{
    if (evento.key==="Enter"){
      let a = document.getElementById("comando").value;
      let b=main(a);
      document.getElementById("historial").innerHTML += "terminal 1.0: " + a + "<br>" + b + "<br>";
      document.getElementById("comando").value="";
    }
  });
}
terminal();
