/* eslint-disable sort-keys */

import http from "k6/http";

export const options = {
  vus: 10,
  duration: "30s"
};

// eslint-disable-next-line prefer-arrow/prefer-arrow-functions
export default function() {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const res = http.get(
    "http://localhost:7071/api/v1/services/01EHA20ZFP101CWMY2PYPEFVPR"
  );
}