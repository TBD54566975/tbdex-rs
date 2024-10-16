import { Router } from 'express';
import { GetOfferingsResponseBody } from 'tbdex';
import { OfferingRepository } from '../data/offering-repository';

export default function offeringsRouter(offeringsRepository: OfferingRepository) {
  const router = Router();

  router.get('/', (req, res) => {
    console.log('GET /offerings');

    const offerings = offeringsRepository.getOfferings();
    const responseBody = new GetOfferingsResponseBody(offerings);

    res.type('application/json');
    res.send(responseBody.toJSONString());
  });

  return router;
}