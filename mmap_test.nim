import os
import memfiles
import strutils


type
  Test* = object
    mmap: ptr array[0..0, uint8]
    size: uint

proc newTest(filename: string): Test =
  var mmap: MemFile = open(filename, mode = fmRead, mappedSize = -1)
  var mmap_mem = cast[ptr array[0..0, uint8]](mmap.mem)
  var size = uint(mmap.size)

  return Test(mmap: mmap_mem, size: size)


proc get(self: Test, loc: uint): uint64 =
  var bounded_loc = loc mod self.size
  var end_byte = self.mmap[bounded_loc]
  var v  = uint64(end_byte)
  return v shr (7.uint - ((bounded_loc - 1) and 7))


proc nextRandom(n: uint): uint =
  var x  = uint32(n)
  x = x xor (x shl 13)
  x = x xor (x shr 17)
  x = x xor (x shl 5)
  return uint(x)


var filename = paramStr(1)
var n_samples = parseInt(paramStr(2))

var test = newTest(filename)
var r = 0.uint64
var i = 1.uint

for _ in 0..n_samples:
  r += test.get(i)
  i = nextRandom(i)

echo r
