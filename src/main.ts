import './style.css'
import init, * as engine from './engine/engine';

await init();

console.log(engine.add(-512n, 1024n));

let app = document.getElementById("app")! as HTMLDivElement;

app.innerHTML = `Hello World!`