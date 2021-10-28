(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./apps/myrender.js":
/*!**************************!*\
  !*** ./apps/myrender.js ***!
  \**************************/
/*! exports provided: WIDTH_SIZE, HEIGHT_SIZE, WIDTH, HEIGHT, default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"WIDTH_SIZE\", function() { return WIDTH_SIZE; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"HEIGHT_SIZE\", function() { return HEIGHT_SIZE; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"WIDTH\", function() { return WIDTH; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"HEIGHT\", function() { return HEIGHT; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"default\", function() { return SimRenderer; });\n\r\nconst WIDTH_SIZE = 100;\r\nconst HEIGHT_SIZE = 220;\r\n\r\nconst WIDTH = 60;\r\nconst HEIGHT = 35;\r\nconst ALIVE_COL = '#fae';\r\nconst DEAD_COL = '#0'\r\n\r\nclass SimRenderer {\r\n    constructor(sim,font) {\r\n        this.sim = sim\r\n        this.font = font\r\n        this.stats = undefined\r\n        this.current_gen;\r\n    }\r\n    show(p) {\r\n        const world = this.sim.world();\r\n        let max = 0;\r\n        for(const bird of world.birds.filter((bird) => bird.state)) {\r\n            this.showBird(p,bird.x,bird.y,bird.state);\r\n            max = bird.counter;\r\n        }\r\n        \r\n        for(const pipe of world.pipes) {\r\n            this.showPipe(p,pipe.x,pipe.y)\r\n        }\r\n        p.push()\r\n        p.fill('#0');\r\n        p.textFont(this.font);\r\n        p.textSize(30);\r\n        p.text(max,p.width /2 ,p.height / 8)\r\n        p.pop()\r\n        this.showPopNum(p)\r\n        this.showLastStat(p)\r\n    }\r\n\r\n\r\n    showBird(p,x_bird,y_bird,state) {\r\n        p.push()\r\n        p.rectMode(p.CENTER); \r\n        p.translate(x_bird - WIDTH / 2 ,y_bird- HEIGHT / 2)\r\n        p.fill(ALIVE_COL)\r\n        if(!state) {\r\n            p.rotate(p.PI / 2);\r\n        }\r\n        p.rect(WIDTH / 2,HEIGHT / 2,WIDTH, HEIGHT);\r\n        \r\n        p.pop()\r\n    }\r\n\r\n\r\n    showPipe(p,pipe_x,pipe_y) {\r\n        p.push()\r\n        p.fill('#1fd8a2')\r\n        p.rect(pipe_x, 0, WIDTH_SIZE ,pipe_y)\r\n        p.rect(pipe_x, pipe_y + HEIGHT_SIZE, WIDTH_SIZE , p.height - (pipe_y + HEIGHT_SIZE));\r\n        p.pop()\r\n    }\r\n\r\n\r\n    showLastStat(p) {\r\n        if(this.stat) {\r\n            p.push()\r\n            p.fill('#0');\r\n            p.textFont(this.font);\r\n            p.textSize(30);\r\n            p.text(`Current Statistics\\nPrevious max :${this.stat.max().toFixed(2)}\\nPrevious avg:${this.stat.avg().toFixed(2)}\\nPop number ${this.stat.pop_num()}`,p.width / 50  ,p.height / 8 + 50)\r\n            p.pop()\r\n        }\r\n    }\r\n\r\n    showPopNum(p) {\r\n        p.push()\r\n        p.fill('#0');\r\n        p.textFont(this.font);\r\n        p.textSize(30);\r\n        p.text(\"Current Gen : \" + this.sim.current_gen(),p.width / 50  ,p.height / 8)\r\n        p.pop()\r\n    }\r\n    \r\n\r\n    addStatistic(stat) {\r\n        this.stat = stat\r\n    }\r\n\r\n}\n\n//# sourceURL=webpack:///./apps/myrender.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var lib_simulation_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! lib-simulation-wasm */ \"./node_modules/lib-simulation-wasm/lib_simulation_wasm.js\");\n/* harmony import */ var p5__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! p5 */ \"./node_modules/p5/lib/p5.min.js\");\n/* harmony import */ var p5__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(p5__WEBPACK_IMPORTED_MODULE_1__);\n/* harmony import */ var _apps_myrender__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./apps/myrender */ \"./apps/myrender.js\");\n!(function webpackMissingModule() { var e = new Error(\"Cannot find module './assets/ZenKurenaido-Regular.tff'\"); e.code = 'MODULE_NOT_FOUND'; throw e; }());\n\r\n\r\n\r\n\r\n\r\n\r\nconst WINDOW_WIDTH = 1536;\r\nconst WINDOW_HEIGHT = 726;\r\nconst sketch = (p) => {\r\n    let simulation;\r\n    let renderer;\r\n    let font;\r\n  \r\n    let canvas;\r\n    p.preload = () => {\r\n      font = p.loadFont(!(function webpackMissingModule() { var e = new Error(\"Cannot find module './assets/ZenKurenaido-Regular.tff'\"); e.code = 'MODULE_NOT_FOUND'; throw e; }()))\r\n    }\r\n    p.setup = () => {\r\n      canvas = p.createCanvas(WINDOW_WIDTH , WINDOW_HEIGHT);\r\n      simulation = new lib_simulation_wasm__WEBPACK_IMPORTED_MODULE_0__[\"Simulation\"](p.width, p.height);;\r\n      renderer = new _apps_myrender__WEBPACK_IMPORTED_MODULE_2__[\"default\"](simulation,font);\r\n    };\r\n  \r\n    p.draw = () => {\r\n        p.background(51);\r\n        let text = simulation.step()\r\n        if(text) {\r\n          renderer.addStatistic(text);\r\n        }\r\n        renderer.show(p)\r\n        \r\n    };\r\n\r\n    \r\n};\r\n  \r\nnew p5__WEBPACK_IMPORTED_MODULE_1___default.a(sketch);\r\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/lib-simulation-wasm sync recursive":
/*!***********************************************!*\
  !*** ./node_modules/lib-simulation-wasm sync ***!
  \***********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function webpackEmptyContext(req) {\n\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\te.code = 'MODULE_NOT_FOUND';\n\tthrow e;\n}\nwebpackEmptyContext.keys = function() { return []; };\nwebpackEmptyContext.resolve = webpackEmptyContext;\nmodule.exports = webpackEmptyContext;\nwebpackEmptyContext.id = \"./node_modules/lib-simulation-wasm sync recursive\";\n\n//# sourceURL=webpack:///./node_modules/lib-simulation-wasm_sync?");

/***/ })

}]);