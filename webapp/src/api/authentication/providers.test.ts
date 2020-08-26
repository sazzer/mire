import * as api from "./providers";

import nock from "nock";

test("Successfully list providers", async () => {
  nock("http://mire-cd.example.com")
    .defaultReplyHeaders({
      "access-control-allow-origin": "*",
      "Access-Control-Expose-Headers": "Content-Type",
    })
    .get("/authentication")
    .reply(200, ["google", "twitter"]);

  const result = await api.listAuthenticationProviders();

  expect(result).toEqual(["google", "twitter"]);
});
