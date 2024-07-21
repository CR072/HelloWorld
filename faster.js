/**
 * 
 *  More complex yet faster way to log "Hello World".
 *  
**/
let target = "hello world";
let index = 0;
let pointer = 0;
let ans = "";
let chars = "abcdefghijklmnopqrstuvwxyz".split('');
let System = { out: { println: (...args) => console.log(...args) } };

function log(char) {
    return new Promise(resolve => {
        setImmediate(() => {
            let cur = target.split('')[pointer];
            
            if (cur === " ") {
                pointer++;
                ans += " ";
            }

            if (char === cur) {
                ans += char || " ";
                pointer++;
            }

            let toLog = ans + char;

            System.out.println(toLog.endsWith("dd") ? toLog.slice(0, -1) : toLog);

            if (ans === target) {
                System.out.println("Successfully logged 'hello world!'");
                process.exit();
            }

            resolve();
        });
    });
}

async function main() {
    while (pointer < target.length) {
        let i = chars[index];
        await log(i);
        index = (index + 1) % chars.length;
    }
}

main().catch(err => console.error(err));
