import axios from 'axios';


export const runHappyPathFlow = async (
  pfiDidUri: string,
  verifiableCredential: string,
  replyToUrl: string
) => {
  console.log('\n ~Running Happy Path Webhook Flow~ \n');

  // TODO: Implement the flow logic
  console.log('1. Fetching offerings...');
  try {
    const response = await axios.get(`${pfiDidUri}/offerings`);
    const offerings = response.data.offerings;
    console.log('Successfully fetched offerings:', offerings);

    // Continue with the flow...
  } catch (error) {
    console.error('Error fetching offerings:', error);
  }
};
