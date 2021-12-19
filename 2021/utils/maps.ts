
// For adding a clone function to Set
// https://stackoverflow.com/q/38434337/1035008
declare global {
    interface Array<T> {
        toMap<K, V>(): Map<K, V>
    }
}

Array.prototype.toMap = function () {
    let map = new Map();
    for (let element of this) {
        let [key, value] = element
        map.set(key, value);
    }
    return map
}

export { };
