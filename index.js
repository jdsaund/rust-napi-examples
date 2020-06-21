const assert = require('assert')
const { reverseString } = require('./index.node')

const str = 'hello world'
const reversed = reverseString(str)
assert(str.split('').reverse().join('') === reversed)
