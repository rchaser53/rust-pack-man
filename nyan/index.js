var ffi = require('ffi');

var lib = ffi.Library('target/release/nyan', {
  'nyan': ['void', []]
});

lib.nyan()