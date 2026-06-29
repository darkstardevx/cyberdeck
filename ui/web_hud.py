from http.server import BaseHTTPRequestHandler, HTTPServer
import json

FILE = "/tmp/cyberdeck_metrics.json"

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type','application/json')
        self.end_headers()

        try:
            data = open(FILE).read()
        except:
            data = '{"status":"no daemon"}'

        self.wfile.write(data.encode())

HTTPServer(("0.0.0.0", 9150), Handler).serve_forever()
