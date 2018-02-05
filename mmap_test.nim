import os
import memfiles
import strutils

import bitarray


proc get(self: var Bitarray, loc: uint): uint64 =
  let lower = int((loc mod uint(self.size_bits - 64)))
  let upper = lower + 64
  let slice = self[lower..upper]
  return uint64(slice)


proc nextRandom(n: uint): uint =
  var x  = uint32(n)
  x = x xor (x shl 13)
  x = x xor (x shr 17)
  x = x xor (x shl 5)
  return uint(x)


var filename = paramStr(1)
var n_samples = parseInt(paramStr(2))

var test = createBitarray(filename, enforceHeader=false)
var r = 0.uint64
var i = 1.uint

for _ in 0..<n_samples:
  r += test.get(i)
  i = nextRandom(i)

echo r
