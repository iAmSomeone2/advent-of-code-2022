export default class FsEntry {
    name = "";
    isDir = false;
    size = 0;
    parent = null;
    children = new Map();

    constructor(name, isDir = true, size = 0) {
        this.name = name;
        this.isDir = isDir;
        this.size = size;
    }

    /**
     * Add child entry to this instance
     * @param {FsEntry} child 
     */
    addChild(child) {
        if (!this.isDir) {
            // Files can't have children
            return;
        }

        this.children.set(child.name, child);
        child.parent = this;
    }
}