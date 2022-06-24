(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/gates_hdl.js":
/*!***************************!*\
  !*** ../pkg/gates_hdl.js ***!
  \***************************/
/*! exports provided: compile_compose, generate_mermaid, __wbindgen_string_new */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./gates_hdl_bg.wasm */ \"../pkg/gates_hdl_bg.wasm\");\n/* harmony import */ var _gates_hdl_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./gates_hdl_bg.js */ \"../pkg/gates_hdl_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"compile_compose\", function() { return _gates_hdl_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"compile_compose\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"generate_mermaid\", function() { return _gates_hdl_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"generate_mermaid\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return _gates_hdl_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_string_new\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/gates_hdl.js?");

/***/ }),

/***/ "../pkg/gates_hdl_bg.js":
/*!******************************!*\
  !*** ../pkg/gates_hdl_bg.js ***!
  \******************************/
/*! exports provided: compile_compose, generate_mermaid, __wbindgen_string_new */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"compile_compose\", function() { return compile_compose; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"generate_mermaid\", function() { return generate_mermaid; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return __wbindgen_string_new; });\n/* harmony import */ var _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./gates_hdl_bg.wasm */ \"../pkg/gates_hdl_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0;\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(_gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0;\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(_gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedInt32Memory0;\n}\n\nfunction getObject(idx) { return heap[idx]; }\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n/**\n* @param {string} src\n* @returns {string}\n*/\nfunction compile_compose(src) {\n    try {\n        const retptr = _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        const ptr0 = passStringToWasm0(src, _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        const len0 = WASM_VECTOR_LEN;\n        _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"compile_compose\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        var r2 = getInt32Memory0()[retptr / 4 + 2];\n        var r3 = getInt32Memory0()[retptr / 4 + 3];\n        var ptr1 = r0;\n        var len1 = r1;\n        if (r3) {\n            ptr1 = 0; len1 = 0;\n            throw takeObject(r2);\n        }\n        return getStringFromWasm0(ptr1, len1);\n    } finally {\n        _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr1, len1);\n    }\n}\n\n/**\n* @param {string} src\n* @returns {string}\n*/\nfunction generate_mermaid(src) {\n    try {\n        const retptr = _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        const ptr0 = passStringToWasm0(src, _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        const len0 = WASM_VECTOR_LEN;\n        _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"generate_mermaid\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        var r2 = getInt32Memory0()[retptr / 4 + 2];\n        var r3 = getInt32Memory0()[retptr / 4 + 3];\n        var ptr1 = r0;\n        var len1 = r1;\n        if (r3) {\n            ptr1 = 0; len1 = 0;\n            throw takeObject(r2);\n        }\n        return getStringFromWasm0(ptr1, len1);\n    } finally {\n        _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        _gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr1, len1);\n    }\n}\n\nfunction __wbindgen_string_new(arg0, arg1) {\n    const ret = getStringFromWasm0(arg0, arg1);\n    return addHeapObject(ret);\n};\n\ncachedInt32Memory0 = new Int32Array(_gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\ncachedUint8Memory0 = new Uint8Array(_gates_hdl_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/gates_hdl_bg.js?");

/***/ }),

/***/ "../pkg/gates_hdl_bg.wasm":
/*!********************************!*\
  !*** ../pkg/gates_hdl_bg.wasm ***!
  \********************************/
/*! exports provided: memory, compile_compose, generate_mermaid, __wbindgen_add_to_stack_pointer, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./gates_hdl_bg.js */ \"../pkg/gates_hdl_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/gates_hdl_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var gates_hdl__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! gates-hdl */ \"../pkg/gates_hdl.js\");\n\n\n// compiler constants\nconst srcInputTextObject = document.getElementById(\"compileInput\");\nconst compileBtn = document.getElementById(\"compileBtn\");\nconst composeOutputTextObject = document.getElementById(\"composeOutput\");\n\nvar insertSvg = function (svgCode, bindFunctions) {\n    element.innerHTML = svgCode;\n}\n\nfunction compileSource(source) {\n    var output = gates_hdl__WEBPACK_IMPORTED_MODULE_0__[\"compile_compose\"](source);\n\n    return output;\n}\n\nfunction generateMermaidInput(source) {\n    var output = gates_hdl__WEBPACK_IMPORTED_MODULE_0__[\"generate_mermaid\"](source);\n\n    return output;\n}\n\ncompileBtn.addEventListener(\"click\", event => {\n    var source = srcInputTextObject.value;\n    var composeOutput = compileSource(source);\n    var generatedMermaid = generateMermaidInput(source);\n    composeOutputTextObject.textContent = composeOutput;\n    mermaid.render('graphDiv', generatedMermaid, insertSvg);\n});\n\n// set defaults\nsrcInputTextObject.value = `DEFINE first_sum AS xor;\nDEFINE first_carry AS and;\nDEFINE second_sum AS xor;\nDEFINE second_carry AS and;\nDEFINE output AS or;\nLINK first_sum -> b OF second_sum;\nLINK first_sum -> a OF second_carry;\nLINK first_carry -> b OF output;`;\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);