from flask import Flask, jsonify
from flask_sslify import SSLify
import ssl

app = Flask(__name__)
sslify = SSLify(app)


context = ssl.SSLContext()
context.load_cert_chain('', 'privkey.pem')
 


app = Flask(__name__)


@app.route('/')
def index():
    return 'Flask is running!'


@app.route('/data')
def names():
    data = {"names": ["John", "Jacob", "Julie", "Jennifer"]}
    return jsonify(data)


#if __name__ == '__main__':
#    app.run()
if __name__ == '__main__':  
     app.run(host='127.0.0.1', debug=True, ssl_context=context)


