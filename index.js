const assert = require('assert')
const { reverseString } = require('./napi-functions.node')
const { MyObject } = require('./napi-classes.node')

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
