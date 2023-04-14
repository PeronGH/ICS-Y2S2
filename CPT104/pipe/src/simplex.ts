// Step 1: Create readable and writable streams
const { readable, writable } = new TransformStream();

// Step 2: Write data to the writable stream
async function writeData() {
  const message = "Hello, Pipe!";
  const encoder = new TextEncoder();
  const writer = writable.getWriter();
  await writer.write(encoder.encode(message));
  writer.releaseLock();
}

// Step 3: Read data from the readable stream
async function readData() {
  const decoder = new TextDecoder();
  const reader = readable.getReader();
  const { value, done } = await reader.read();
  const receivedMessage = decoder.decode(value);

  console.log("Received message:", receivedMessage);

  // Step 4: Close the readable stream
  if (!done) {
    await reader.cancel();
  }
}

await Promise.all([writeData(), readData()]);
