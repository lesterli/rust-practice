// in `print.js`

const { argv, stdout } = process;

// we have to skip *two* arguments: the path to node,
// and the path to our script
for (const arg of argv.slice(2)) {
    for (const character of arg) {
        stdout.write(character);
        stdout.write(" ");
    }
    stdout.write("\n");
}