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
        const sharedArray = new Int32Array(require('worker_threads').workerData.sharedBuffer);

        parentPort.on('message', async (url) => {
          try {
            const response = await fetch(url);
            const statusCode = response.status;

            Atomics.store(sharedArray, 0, statusCode);
            Atomics.notify(sharedArray, 0);
          } catch (error) {
            console.error('Worker fetch error:', error);
          }
        });
      `;

      if (module.isMainThread) {
        const worker = new module.Worker(workerCode, {
          eval: true,
          workerData: { sharedBuffer },
        });

        worker.postMessage("https://example.com");

        Atomics.wait(sharedArray, 0, 0);

        const statusCode = Atomics.load(sharedArray, 0);
        console.log("fetch response status code", statusCode);
      }
    })
    .catch((err) => console.error("Failed to load Node module:", err));
} else {
  const workerCode = `
    onmessage = async (event) => {
      const url = event.data;
      try {
        const response = await fetch(url);
        const statusCode = response.status;
        postMessage(statusCode); 
      } catch (error) {
        console.error('Worker fetch error:', error);
        postMessage({ status: 'Error', message: error.message });
      }
    };
  `;

  const blob = new Blob([workerCode], { type: "application/javascript" });
  const worker = new Worker(URL.createObjectURL(blob));

  const waitForWorker = () => {
    return new Promise((resolve) => {
      worker.onmessage = (event) => {
        console.log("fetch response status code", event.data);
        resolve(undefined);
      };
    });
  };

  const run = async () => {
    try {
      worker.postMessage("https://jsonplaceholder.typicode.com/todos/1");
      await waitForWorker();
      worker.terminate();
    } catch (err) {
      console.error(err);
    }
  };

  await run();
}
