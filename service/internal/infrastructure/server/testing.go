package server

import "net/http"

// ServeHTTP will serve the incoming request to the HTTP Server and write the response onto the Response Writer.
func (s *Server) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	s.router.ServeHTTP(w, req)
}
