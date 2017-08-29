var ffi = require('ffi');

var lib = ffi.Library('target/release/guessing_game', {
  'process': ['void', []]
});

lib.process();

console.log("done!");