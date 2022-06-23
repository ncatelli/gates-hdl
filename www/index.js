import * as wasm from "gates-hdl";

// compiler constants
const srcInputTextObject = document.getElementById("compileInput");
const compileBtn = document.getElementById("compileBtn");
const composeOutputTextObject = document.getElementById("composeOutput");

function download(filename, contentType, data) {
    var element = document.createElement('a');
    element.setAttribute('href', contentType + encodeURIComponent(data));
    element.setAttribute('download', filename);

    element.style.display = 'none';
    document.body.appendChild(element);

    element.click();

    document.body.removeChild(element);
}

function compileSource(source) {
    var output = wasm.compile_compose(source);

    return output;
}

compileBtn.addEventListener("click", event => {
    var source = srcInputTextObject.value;
    var composeOutput = compileSource(source);

    // update text area with output
    composeOutputTextObject.textContent = composeOutput;

    // Start file download.
    var composeOutput = Buffer.from(composeOutput).toString();
    download("docker-compose.yaml", 'text/yaml', composeOutput);
});
