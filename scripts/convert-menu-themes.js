const input = prompt("Menu theme data from system.js?");
const themes = JSON.parse(input.replaceAll("'", '"'));
const entries = Object.entries(themes).sort((a, b) => a[0] > b[0]);

let output = "";
for (const [key, values] of entries) {
    output += `${key}:\n`;
    for (const value of values) {
        if (!isNaN(Number(value)))
            output += `  - "${value}"\n`;
        else output += `  - ${value}\n`;
    }
}

console.log(output);
