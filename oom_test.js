export const main = async () => {
    let arr = [];
    for (let i = 1; i > 0; i++) {
        arr.push(new Array(100).fill(0));
    }
    return "Hello world from a JS Fleek Function! -" + arr[0];
}
