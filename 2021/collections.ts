import { textSpanIntersectsWithPosition } from "typescript";

// https://stackoverflow.com/a/51321724/1035008
export class DefaultDict<K, V> extends Map<K, V> {
    defaultFunc: () => V;

    constructor(defaultFunc: (() => V)) {
        super()
        this.defaultFunc = defaultFunc
    }

    override get(key: K): V {
        let value = super.get(key)
        if (value) {
            return value
        } else {
            let defaultValue = this.defaultFunc()
            super.set(key, defaultValue)
            return defaultValue
        }
    }
}

export class Counter<K> extends DefaultDict<K, number> {
    constructor() {
        super(() => 0)
    }

    add(key: K, count: number) {
        super.set(key, super.get(key) + count)
    }
}

// For adding a clone function to Set
// https://stackoverflow.com/q/38434337/1035008
declare global {
    interface Array<T> {
        top(): T | undefined

        sum(): T
    }
}

Array.prototype.top = function () {
    return this.at(this.length - 1)
}

Array.prototype.sum = function () {
    return this.reduce((a, b) => a + b, 0)
}

export { };
