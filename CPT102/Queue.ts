export class Queue<T> {
  #data: T[] = [];
  #queueStart = 0;

  get data(): T[] {
    return this.#data;
  }

  enqueue(...item: T[]) {
    this.#data.push(...item);
  }

  offer(...item: T[]) {
    this.enqueue(...item);
  }

  dequeue(): T | undefined {
    if (this.isEmpty) return undefined;
    const item = this.#data[this.#queueStart];
    delete this.#data[this.#queueStart];
    this.#queueStart++;
    return item;
  }

  poll(): T | undefined {
    return this.dequeue();
  }

  peek(): T | undefined {
    if (this.isEmpty) return undefined;
    return this.#data[this.#queueStart];
  }

  get size(): number {
    return this.#data.length - this.#queueStart;
  }

  get isEmpty(): boolean {
    return this.size === 0;
  }
}
