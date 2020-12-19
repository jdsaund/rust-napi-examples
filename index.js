const assert = require('assert')
const { reverseString } = require('./libnapi_functions.node')
const { MyObject } = require('./libnapi_classes.node')
const { rotateSharedBuffer, rotateArray } = require('./libnapi_parameters.node')

// testing functions
const str = 'hello world'
const reversed = reverseString(str)
assert(str.split('').reverse().join('') === reversed)

// testing classes
const obj = new MyObject()
assert(obj.value === 0)
obj.value = 4
assert(obj.value === 4)
assert(typeof obj.ping === 'function')
obj.ping()

// create a new rotated array
const array = [1, 2, 3]
const arrayResult = rotateArray(array)
console.log(arrayResult)

// rotate a shared typed array in-place
const typedArray = new Uint8Array([1, 2, 3])
rotateSharedBuffer(typedArray)
console.log(typedArray)
