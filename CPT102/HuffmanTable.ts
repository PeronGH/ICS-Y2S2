import { PriorityQueue } from "./PriorityQueue.ts";

const compareCounts = (a: CharCount, b: CharCount) => a.count - b.count;

function countChars(data: string): CharCount[] {
  const lengthMap = new Map<string, number>();

  for (const char of data) {
    const count = lengthMap.get(char) ?? 0;
    lengthMap.set(char, count + 1);
  }

  return [...lengthMap]
    .map(([char, count]) => ({ char, count }))
    .sort(compareCounts);
}

function buildTree(counts: CharCount[]): HuffmanNode {
  const queue = new PriorityQueue<HuffmanNode>(compareCounts);
  queue.enqueue(...counts);

  while (queue.size > 1) {
    const left = queue.dequeue()!;
    const right = queue.dequeue()!;
    queue.enqueue({ count: left.count + right.count, left, right });
  }

  return queue.dequeue()!;
}

function buildCodeBook(tree: HuffmanNode): Codebook {
  const codebook: Codebook = new Map();

  const traverse = (node: HuffmanNode, code = "") => {
    if (node.char) {
      codebook.set(node.char, code);
    } else {
      traverse(node.left!, code + "0");
      traverse(node.right!, code + "1");
    }
  };

  traverse(tree);

  return codebook;
}

Deno.test("build codebook", () => {
  const counts = countChars("abracadabra");
  const tree = buildTree(counts);
  const codebook = buildCodeBook(tree);
  console.log(codebook);

  codebook.forEach((code, char) =>
    console.log(
      char,
      code.length * counts.find((c) => c.char === char)!.count,
    )
  );
});

Deno.test("exercise", () => {
  const counts: CharCount[] = [
    { char: "A", count: 1 },
    { char: "B", count: 3 },
    { char: "C", count: 5 },
    { char: "D", count: 7 },
    { char: "E", count: 11 },
  ];

  const tree = buildTree(counts);
  const codebook = buildCodeBook(tree);

  [...codebook]
    .sort(([a], [b]) => a.localeCompare(b))
    .forEach(([char, code]) => console.log(char, code));
});

interface CharCount {
  char?: string;
  count: number;
}

interface HuffmanNode extends CharCount {
  char?: string;
  count: number;
  left?: HuffmanNode;
  right?: HuffmanNode;
}

type Codebook = Map<string, string>;
