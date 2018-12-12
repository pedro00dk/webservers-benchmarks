from flask import Flask
import logging

log = logging.getLogger('werkzeug')
log.setLevel(logging.ERROR)

app = Flask(__name__)
hello = 'Hello world!'

@app.route('/')
def get():
    return hello

if __name__ == '__main__':
      app.run(host='0.0.0.0', port=8000)