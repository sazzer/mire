package integration

import (
	"io/ioutil"
	"net/http"
	"net/http/httptest"

	"github.com/stretchr/testify/assert"
)

// Response received from injecting a request into the service.
type TestResponse struct {
	StatusCode int
	Header     http.Header
	Body       []byte
}

// Inject an HTTP Request into the service and get the response.
func (s *Service) Inject(r *http.Request) TestResponse {
	w := httptest.NewRecorder()
	s.service.ServeHTTP(w, r)

	res := w.Result()
	defer res.Body.Close()

	body, e := ioutil.ReadAll(res.Body)
	assert.NoError(s.t, e)

	return TestResponse{
		StatusCode: res.StatusCode,
		Header:     res.Header,
		Body:       body,
	}
}
