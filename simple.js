let target = "hello world";
let index = 0;
let pointer = 0
let ans = ""
let chars = "abcdefghijklmnopqrstuvwxyz".split('')
let System = { out: { println: (...args) => console.log(...args) } }

function main() {
    let cur = chars[index]
    index++
    let targetIndex = target.split('')[pointer]
    
    if (targetIndex === " ") {
        pointer++
        ans += " "
    }

    if (cur === targetIndex) {
        ans += cur || " "
        pointer++
    }

    let toLog = ans + cur;

    System.out.println(toLog.endsWith("dd") ? toLog.slice(0, -1) : toLog);

    if (ans == target) {
        System.out.println("Successfully logged Hello World!")
        process.exit()
    }

    setTimeout(() => {
        main()
        if (index >= 26) index = 0
    }, 0);
}
main()
