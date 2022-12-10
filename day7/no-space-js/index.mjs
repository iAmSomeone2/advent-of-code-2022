import fs from "node:fs/promises";
import FsEntry from "./fs-entry.mjs";

/**
 * 
 * @param {FsEntry} currentDir 
 * @param {string} key 
 */
function changeDirectory(currentDir, key) {
    if (key === "..") {
        console.log(`Changing directory up one level (${currentDir.parent.name})`);
        return currentDir.parent;
    }
    console.log(`Changing directory to ${key}`);
    return currentDir.children.get(key);
}

const DIR_ENTRY_REGEX = /^dir (?<name>\w+)/;
const FILE_ENTRY_REGEX = /^(?<size>\d+) (?<name>\w+(\.\w+)?)$/;
const CD_REGEX = /^\$ cd (?<key>\.{2}|\w+)$/;

const inputData = (await fs.readFile("test_input.txt", { encoding: "utf-8" })).split('\n');

let rootDir = new FsEntry("/");
let currentDir = rootDir;

let isListing = false;
inputData.forEach((line, num) => {
    console.debug(`Line number: ${num + 1}`);
    if ((line === "$ cd /") || (line === "")) {
        // Skip the first and last lines
        return;
    }

    if (isListing && line.startsWith("$")) {
        isListing = false;
    }

    if (line === "$ ls") {
        isListing = true;
        return;
    }

    if (isListing) {
        let tmpEntry = null;

        const dirEntryResult = DIR_ENTRY_REGEX.exec(line);
        if (dirEntryResult) {
            tmpEntry = new FsEntry(dirEntryResult.groups.name);
        }

        const fileEntryResult = FILE_ENTRY_REGEX.exec(line);
        if (fileEntryResult) {
            tmpEntry = new FsEntry(
                fileEntryResult.groups.name,
                false,
                Number.parseInt(fileEntryResult.groups.size)
            );
        }

        if (tmpEntry) {
            // console.debug(`Adding child to ${currentDir}`);
            currentDir.addChild(tmpEntry);
        }
        return;
    }

    const cdResult = CD_REGEX.exec(line);
    // console.dir(cdResult);
    if (cdResult) {
        const key = cdResult.groups.key;
        currentDir = changeDirectory(currentDir, key);
    }
});

console.dir(rootDir);
