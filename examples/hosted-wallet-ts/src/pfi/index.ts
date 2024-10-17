import dotenv from 'dotenv';
dotenv.config();

import express from 'express';
import offeringsRouter from './api/offerings.js';
import exchangesRouter from './api/exchanges.js';
import { OfferingRepository } from './data/offering-repository.js';
import { PortableDid, BearerDid } from 'tbdex';

const pfiPortableDid: PortableDid = JSON.parse(process.env.PFI_PORTABLE_DID);
const pfiBearerDid = BearerDid.fromPortableDID(pfiPortableDid);

const app = express();
const PORT = 8082;

app.use(express.json());

const offeringsRepository = new OfferingRepository();
await offeringsRepository.signAndVerifyOfferings(pfiBearerDid);

app.use('/offerings', offeringsRouter(offeringsRepository));
app.use('/exchanges', exchangesRouter(pfiBearerDid, offeringsRepository));

app.listen(PORT, () => {
  console.log(`PFI server running on port ${PORT}`);
});