class MyCache<T> {
    inner: T | null

    public constructor() {
        this.inner = null
    }

    public fetch(func: () => T): T {
        if(this.inner == null) {
            this.inner = func();
        }
        return this.inner
    }
}

const cache = new Map<string, MyCache<any>>();

export function cacheCheck<T>(name: string, or: () => T): T {
    if(!cache.has(name)) {
        cache.set(name, new MyCache())
    }
    return cache.get(name).fetch(or)
}
