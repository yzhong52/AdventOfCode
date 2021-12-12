
// https://stackoverflow.com/a/51321724/1035008
export class DefaultDict<K, V> extends Map {
    defaultFunc: () => V;

    constructor(defaultFunc: (() => V)) {
        super()
        this.defaultFunc = defaultFunc
    }

    override get(key: K): V {
        if (!super.has(key)) {
            super.set(key, this.defaultFunc())
        }
        return super.get(key)
    }
}
