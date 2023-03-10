export class PriorityQueue<T> {
  private _queue: T[] = [];
  private _comparator: (a: T, b: T) => number;

  constructor(comparator: (a: T, b: T) => number) {
    this._comparator = comparator;
  }

  public enqueue(...item: T[]) {
    this._queue.push(...item);
    this._queue.sort(this._comparator);
  }

  public dequeue(): T | undefined {
    return this._queue.shift();
  }

  public peek(): T | undefined {
    return this._queue[0];
  }

  public get size(): number {
    return this._queue.length;
  }
}
