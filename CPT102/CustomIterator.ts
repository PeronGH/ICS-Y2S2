export class CustomIterator<T> implements Iterator<T> {
  #data: T[];
  #index: number;

  constructor(...data: T[]) {
    this.#data = data;
    this.#index = data.length - 1;
  }

  next(): IteratorResult<T> {
    if (this.#index < 0) return { done: true, value: undefined };
    const item = this.#data[this.#index];
    this.#index--;
    return { done: false, value: item };
  }

  [Symbol.iterator](): Iterator<T> {
    return this;
  }
}

export function* customIterator<T>(...data: T[]) {
  for (let i = data.length - 1; i >= 0; i--) {
    yield data[i];
  }
}
