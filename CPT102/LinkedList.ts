import { assertEquals } from "https://deno.land/std@0.182.0/testing/asserts.ts";

class Node<E> {
  value: E;
  next?: Node<E>;

  constructor(value: E, next?: Node<E>) {
    this.value = value;
    this.next = next;
  }
}

interface LinkedList<E> extends Iterable<E> {
  readonly length: number;
  head?: Node<E>;
  at(index: number): E | undefined;
  set(index: number, value: E): boolean;
  addAt(index: number, value: E): boolean;

  [index: number]: E;
}

class InternalLinkedList<E> implements LinkedList<E> {
  [index: number]: E;

  head?: Node<E>;
  length = 0;

  [Symbol.iterator]() {
    return new LinkedListIterator(this.head);
  }

  // ensure index is in range, otherwise return -1
  handleIndex(index: number): number {
    if (index < 0) {
      index += this.length;
    }

    if (index < 0 || index >= this.length) {
      return -1;
    }

    return index;
  }

  at(index: number): E | undefined {
    index = this.handleIndex(index);
    if (index === -1) return undefined;

    let current = this.head;
    for (let i = 0; i < index; i++) {
      current = current?.next;
    }

    return current!.value;
  }

  set(index: number, value: E): boolean {
    index = this.handleIndex(index);
    if (index === -1) return false;

    let current = this.head;
    for (let i = 0; i < index; i++) {
      current = current?.next;
    }

    current!.value = value;

    return true;
  }

  addAt(index: number, value: E): boolean {
    if (index < 0 || index > this.length) return false;

    const newNode = new Node(value);

    if (index === 0) {
      newNode.next = this.head;
      this.head = newNode;
    } else {
      let current = this.head;
      for (let i = 0; i < index - 1; i++) {
        current = current?.next;
      }
      newNode.next = current?.next;
      if (current) {
        current.next = newNode;
      }
    }

    this.length++;
    return true;
  }
}

class LinkedListIterator<E> implements Iterator<E> {
  #current?: Node<E>;

  constructor(head?: Node<E>) {
    this.#current = head;
  }

  next(): IteratorResult<E> {
    if (!this.#current) {
      return { done: true, value: undefined };
    }

    const { value } = this.#current;
    this.#current = this.#current.next;
    return { done: false, value };
  }
}

export function LinkedList<E>(): LinkedList<E> {
  return new Proxy(new InternalLinkedList<E>(), {
    get(target: InternalLinkedList<E>, prop: PropertyKey) {
      if (typeof prop === "string" && !isNaN(Number(prop))) {
        return target.at(Number(prop));
      } else {
        // deno-lint-ignore no-explicit-any
        return (target as any)[prop];
      }
    },

    set(target: InternalLinkedList<E>, prop: PropertyKey, value: E) {
      if (typeof prop === "string" && !isNaN(Number(prop))) {
        return target.set(Number(prop), value);
      } else {
        // deno-lint-ignore no-explicit-any
        (target as any)[prop] = value;
        return true;
      }
    },
  });
}

Deno.test("LinkedList", () => {
  const list = LinkedList<number>();
  list.addAt(0, 1);
  list.addAt(1, 2);
  list.addAt(2, 3);
  list.addAt(3, 4);
  list.addAt(4, 5);

  assertEquals(list.length, 5);
  assertEquals(list.at(0), 1);
  assertEquals(list.at(1), 2);
  assertEquals(list.at(2), 3);
  assertEquals(list.at(3), 4);
  assertEquals(list.at(4), 5);

  list[0] = 6;
  list[1] = 7;
  list[2] = 8;
  list[3] = 9;
  list[4] = 10;

  assertEquals(list.length, 5);
  assertEquals(list.at(0), 6);
  assertEquals(list.at(1), 7);
  assertEquals(list.at(2), 8);
  assertEquals(list.at(3), 9);
  assertEquals(list.at(4), 10);

  assertEquals([...list], [6, 7, 8, 9, 10]);
});
