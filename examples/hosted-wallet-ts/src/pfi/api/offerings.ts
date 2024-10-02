import { Router } from 'express';

const router = Router();

router.get('/', (req, res) => {
  console.log('GET /offerings');
  // TODO: Implement logic to fetch offerings
  res.json({
    offerings: [
      {
        id: 'offering_01',
        description: 'Fake offering 1',
        // ...other offering details
      }
    ]
  });
});

export default router;
