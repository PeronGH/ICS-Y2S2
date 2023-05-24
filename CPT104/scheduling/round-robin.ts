interface Process {
  id: number;
  burst: number;
  arrival: number;
}

interface ProcessExecution extends Process {
  remaining: number;
  start?: number;
  finish?: number;
}

function roundRobin(processes: Process[], quantum: number): ProcessExecution[] {
  let time = 0;
  const queue: ProcessExecution[] = [];
  const executedProcesses: ProcessExecution[] = [];

  // Create a copy of the processes array to avoid modifying the original
  let remainingProcesses = processes.map((process) => ({
    ...process,
    remaining: process.burst,
  }));

  // Sort processes by arrival time
  remainingProcesses.sort((a, b) => a.arrival - b.arrival);

  while (remainingProcesses.length > 0 || queue.length > 0) {
    // Add processes that have arrived to the queue
    remainingProcesses = remainingProcesses.filter((process) => {
      if (process.arrival <= time) {
        queue.push(process);
        return false;
      }
      return true;
    });

    // If the queue is empty, move time forward
    if (queue.length === 0) {
      time = remainingProcesses[0].arrival;
      continue;
    }

    // Get the next process from the queue
    const process = queue.shift() as ProcessExecution;

    // Record the start time of the process
    if (process.start === undefined) {
      process.start = time;
    }

    // Execute the process
    if (process.remaining > quantum) {
      time += quantum;
      process.remaining -= quantum;
      queue.push(process);
    } else {
      time += process.remaining;
      process.finish = time;
      executedProcesses.push(process);
    }
  }

  return executedProcesses;
}

// Example usage:
const processes: Process[] = [
  { id: 1, burst: 53, arrival: 0 },
  { id: 2, burst: 17, arrival: 0 },
  { id: 3, burst: 68, arrival: 0 },
  { id: 4, burst: 24, arrival: 0 },
];
const quantum = 20;
const executedProcesses = roundRobin(processes, quantum);

// Calculate and print the turnaround time and waiting time for each process
for (const process of executedProcesses) {
  const turnaroundTime = process.finish! - process.arrival;
  const waitingTime = turnaroundTime - process.burst;
  console.log(
    `Process ${process.id}: Turnaround time = ${turnaroundTime}, Waiting time = ${waitingTime}`,
  );
}
