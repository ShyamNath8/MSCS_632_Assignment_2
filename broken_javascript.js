// JavaScript: intentionally broken
function calculateSum(arr) {
    let total = 0;
    for (let num of arr {   // <-- missing closing parenthesis before the brace
        total += num;
    }
    return total;
}

let numbers = [1, 2, 3, 4, 5];
console.log("Sum in JavaScript:", calculateSum(numbers));
