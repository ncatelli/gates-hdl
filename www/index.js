import * as wasm from "gates-hdl";

// compiler constants
const srcInputTextObject = document.getElementById("compileInput");
const compileBtn = document.getElementById("compileBtn");
const composeOutputTextObject = document.getElementById("composeOutput");

function compileSource(source) {
    var output = wasm.compile_compose(source);

    return output;
}

compileBtn.addEventListener("click", event => {
    var source = srcInputTextObject.value;
    var composeOutput = compileSource(source);

    // update text area with output
    composeOutputTextObject.textContent = composeOutput;
});
