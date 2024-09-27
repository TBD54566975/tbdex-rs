export {};

const isNode =
  typeof process !== "undefined" &&
  process.versions != null &&
  process.versions.node != null;

if (isNode) {
  import("worker_threads")
    .then((module) => {
      const sharedBuffer = new SharedArrayBuffer(4);
      const sharedArray = new Int32Array(sharedBuffer);

      const workerCode = `
        const { parentPort } = require('worker_threads');
        const { setTimeout } = require('timers/promises');
        const sharedArray = new Int32Array(require('worker_threads').workerData.sharedBuffer);

        parentPort.on('message', async (message) => {
          console.log(\`Worker received: \${message}\`);

          // Simulate async operation with a delay
          console.log('Worker: starting async task');
          await setTimeout(2000); // Simulate async work
          console.log('Worker: async task completed');

          // Signal completion by setting atomic value
          Atomics.store(sharedArray, 0, 1); // Set the value to 1
          Atomics.notify(sharedArray, 0); // Notify waiting threads

          parentPort.postMessage('Hello from Worker Thread');
        });
      `;

      if (module.isMainThread) {
        const worker = new module.Worker(workerCode, {
          eval: true,
          workerData: { sharedBuffer },
        });

        worker.postMessage("Hello from Main Thread");

        console.log("Main thread: waiting for worker...");
        Atomics.wait(sharedArray, 0, 0);

        worker.on("message", (message) => {
          console.log(`Main thread received: ${message}`);
        });

        console.log("Main thread: worker has completed its async task");
      }
    })
    .catch((err) => console.error("Failed to load Node module:", err));
} else {
  const workerCode = `
    onmessage = async (event) => {
      try {
        console.log(\`Worker received: \${event.data}\`);

        // Simulate async operation with a delay
        console.log('Worker: starting async task');
        await new Promise(resolve => setTimeout(resolve, 2000)); // Simulate async work
        console.log('Worker: async task completed');

        // Send message back to main thread
        postMessage('Hello from Worker Thread');
      } catch (error) {
        console.error('Worker error:', error);
        postMessage('Worker encountered an error');
      }
    };
  `;

  const blob = new Blob([workerCode], { type: "application/javascript" });
  const worker = new Worker(URL.createObjectURL(blob));

  const waitForWorker = () => {
    return new Promise((resolve) => {
      worker.onmessage = (event) => {
        console.log(`Main thread received: ${event.data}`);
        console.log("Main thread: worker has completed its async task");
        resolve(undefined);
      };
    });
  };

  const run = async () => {
    try {
      console.log("Main thread: sent message to worker");
      worker.postMessage("Hello from Main Thread");
      console.log("Posted message...");

      await waitForWorker();
      console.log("Main thread: worker task completed, proceeding...");
      worker.terminate();

      await new Promise((resolve) => setTimeout(resolve, 3000));
    } catch (err) {
      console.error(err);
    }
  };

  await run();
}
