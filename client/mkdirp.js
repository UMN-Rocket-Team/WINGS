// minimal cross-platform mkdir -p
// used by pnpm test script

const fs = require('fs');
const path = require('path');

if (process.argv.length < 2) {
    console.log('usage: node mkdirp [folders ...]');
    process.exit(1);
}

for (let i = 2; i < process.argv.length; i++) {
    const folderName = process.argv[i];
    const resolved = path.resolve(folderName);
    fs.mkdirSync(resolved, {
        recursive: true
    });
}
