function appearUnique(char: string, text: string) {
  let isUnique = false;

  for (const c of text) {
    if (c === char) {
      if (isUnique) return false;
      else isUnique = true;
    }
  }

  return isUnique;
}

console.log(appearUnique("a", "abc"));
console.log(appearUnique("a", "banana"));
