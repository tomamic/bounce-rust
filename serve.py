#!/usr/bin/env python3
import http.server, socketserver

PORT = 8000

handler = http.server.SimpleHTTPRequestHandler
handler.extensions_map = {
    '.manifest': 'text/cache-manifest',
    '.html': 'text/html',
    '.png': 'image/png',
    '.jpg': 'image/jpg',
    '.svg': 'image/svg+xml',
    '.css': 'text/css',
    '.js':  'application/x-javascript',
    '': 'application/octet-stream', # Default
    }

httpd = socketserver.TCPServer(("", PORT), handler)
print("serving at port", PORT)
httpd.serve_forever()

