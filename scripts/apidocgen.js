const fs = require("fs");
const path = require("path");

function formatAsTable(mappings, maxColLength = 16) {
  let table = "| Variants | &nbsp; | &nbsp; |\n";
  table +=
    "|-------------------|-----------------------|-----------------------|\n";
  const numCols = 3;
  const numRows = Math.ceil(mappings.length / numCols);

  for (let i = 0; i < numRows; i++) {
    table += "|";
    for (let j = 0; j < numCols; j++) {
      const index = j * numRows + i;
      if (index < mappings.length) {
        table += ` **${mappings[index]}** |`;
      } else {
        table += " |";
      }
    }
    table += "\n";
  }

  return table;
}

function processFile(filePath) {
  const data = fs.readFileSync(filePath, "utf8");
  const fromImplRegex = /impl From<(\w+)> for &str[\s\S]*?\{([\s\S]*?)\}/g;
  let match;
  let output = "";

  const heading = path.basename(path.dirname(filePath));
  const capitalizedHeading = heading.charAt(0).toUpperCase() + heading.slice(1);
  output += `### ${capitalizedHeading}\n\n`;

  while ((match = fromImplRegex.exec(data)) !== null) {
    const enumName = match[1].toLowerCase();
    const implBlock = match[2];
    const mappingRegex = /=>\s*"([^"]*)"/g;
    let mappingMatch;
    let mappings = [];

    while ((mappingMatch = mappingRegex.exec(implBlock)) !== null) {
      mappings.push(mappingMatch[1]);
    }

    output += `#### \`${enumName}:\`\n`;
    output += formatAsTable(mappings);
    output += "\n";
  }

  return output;
}

function searchTypesFiles(dirPath) {
  let results = [];
  const files = fs.readdirSync(dirPath);

  files.forEach((file) => {
    const fullPath = path.join(dirPath, file);
    const stat = fs.statSync(fullPath);

    if (stat && stat.isDirectory()) {
      results = results.concat(searchTypesFiles(fullPath));
    } else if (file === "types.rs") {
      results.push(fullPath);
    }
  });

  return results;
}

function main(dirPath) {
  const typesFiles = searchTypesFiles(dirPath);
  typesFiles.forEach((file) => {
    const output = processFile(file);
    console.log(output);
  });
}

if (process.argv.length < 3) {
  console.error("Please provide a directory path as an argument.");
  process.exit(1);
}

const dirPath = process.argv[2];
main(dirPath);
