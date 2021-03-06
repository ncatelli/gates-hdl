import * as wasm from "gates-hdl";

// compiler constants
const srcInputTextObject = document.getElementById("compileInput");
const compileBtn = document.getElementById("compileBtn");
const composeOutputTextObject = document.getElementById("composeOutput");

var insertSvg = function (svgCode, bindFunctions) {
    element.innerHTML = svgCode;
}

function compileSource(source) {
    var output = wasm.compile_compose(source);

    return output;
}

function generateMermaidInput(source) {
    var output = wasm.generate_mermaid(source);

    return output;
}

compileBtn.addEventListener("click", event => {
    var source = srcInputTextObject.value;
    var composeOutput = compileSource(source);
    var generatedMermaid = generateMermaidInput(source);
    composeOutputTextObject.textContent = composeOutput;
    mermaid.render('graphDiv', generatedMermaid, insertSvg);
});

// set defaults
srcInputTextObject.value = `DEFINE first_sum AS xor;
DEFINE first_carry AS and;
DEFINE second_sum AS xor;
DEFINE second_carry AS and;
DEFINE output AS or;
LINK first_sum -> b OF second_sum;
LINK first_sum -> a OF second_carry;
LINK first_carry -> b OF output;`;
