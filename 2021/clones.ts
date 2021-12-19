
// For adding a clone function to Set
// https://stackoverflow.com/q/38434337/1035008 
declare global {
    interface Set<T> {
        clone(): Set<T>
    }
}

Set.prototype.clone = function () {
    let newSet = new Set();
    for (let element of this) {
        newSet.add(element)
    }
    return newSet
}

export { };
