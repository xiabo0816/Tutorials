# -*- coding: utf-8 -*-
from flask import Flask
from flask import request
import json
import argparse

app = Flask(__name__)

def get_args_parser():
    parser = argparse.ArgumentParser(description='http-service')
    parser.add_argument('-p', '--port', default=8080, type=int, help='监听端口')
    return parser.parse_args()

@app.route('/', methods=['GET', 'POST'])
def aid():
    args = request.args
    return json.dumps(args, ensure_ascii=False)


# curl http://127.0.0.1:8080/\?a=1\&b=2
if __name__ == '__main__':
    args = get_args_parser()
    app.run(host='0.0.0.0', port=args.port, debug=True)
