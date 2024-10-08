import express from 'express';
import dotenv from 'dotenv';
import offeringsRouter from './api/offerings.js';
import exchangesRouter from './api/exchanges.js';

dotenv.config();

const app = express();
const PORT = 8082;

app.use(express.json());

// Routes
app.use('/offerings', offeringsRouter);
app.use('/exchanges', exchangesRouter);

app.listen(PORT, () => {
  console.log(`PFI server running on port ${PORT}`);
});
