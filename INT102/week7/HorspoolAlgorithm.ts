// Build the shift table for the pattern string
function buildShiftTable(pattern: string): Map<string, number> {
  const shiftTable = new Map<string, number>();
  const patternLength = pattern.length;

  // Calculate the shift values for each character in the pattern except the last one
  for (let i = 0; i < patternLength - 1; i++) {
    shiftTable.set(pattern[i], patternLength - i - 1);
  }

  console.log(shiftTable);
  return shiftTable;
}

// Horspool's algorithm function to search for the pattern in the text
function horspool(text: string, pattern: string): number | null {
  const textLength = text.length;
  const patternLength = pattern.length;
  // Preprocessing: Build the shift table for the pattern
  const shiftTable = buildShiftTable(pattern);

  // Initialize the index variable to traverse the text
  let i = 0;

  // Continue searching until the end of the text is reached or the pattern is found
  while (i <= textLength - patternLength) {
    // Start comparing characters from the right end of the pattern
    let j = patternLength - 1;

    // Compare characters while they match
    while (j >= 0 && pattern[j] === text[i + j]) {
      j--;
    }

    // If j is less than 0, the pattern has been found at position i
    if (j < 0) {
      return i;
    } else {
      // If there is a mismatch, use the shift table to determine how much to shift the pattern
      const shift = shiftTable.get(text[i + patternLength - 1]) ??
        patternLength;
      i += shift;
    }
  }

  // If the pattern is not found in the text, return null
  return null;
}

// Example usage
const text = "The quick brown fox jumps over the lazy dog";
const pattern = "jumps";

const position = horspool(text, pattern);

if (position !== null) {
  console.log(`Pattern found at position: ${position}`);
} else {
  console.log("Pattern not found");
}
