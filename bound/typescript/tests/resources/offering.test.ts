import { expect } from "chai";
import {scaffoled} from "../../src/resources/offering"

describe('scaffold', () => {
  it('should return 1', async () => {
    let value = scaffoled()
    expect(value).to.equal(1)
  });
})