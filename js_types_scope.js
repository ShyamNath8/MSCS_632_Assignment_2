// Dynamic typing
function adder(x, y) {
  return x + y;
}
console.log(adder(2, 3));          // 5
console.log(adder("a", "b"));      // "ab"

// Closure and lexical scope
function makeCounter() {
  let count = 0;
  return function () { // closure captures `count`
    count += 1;
    return count;
  };
}
const c1 = makeCounter();
console.log(c1()); // 1
console.log(c1()); // 2
