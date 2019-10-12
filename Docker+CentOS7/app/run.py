# -*- coding: utf-8 -*-

# Installation
# pip install pandas
# pip install PyMySQL
import pandas as pd
import pymysql
from flask import Flask
from flask import request
import json
import argparse

app = Flask(__name__)
conn = pymysql.connect(host="XXX", port=5258,
                       user="XXX", passwd="XXX", db="XXX")
cursor = conn.cursor()


def get_args_parser():
    parser = argparse.ArgumentParser(description='中文数据全文')
    parser.add_argument('-p', '--port', default=8080,
                        type=int, help='监听端口')
    return parser.parse_args()


def strB2Q(bs):
    rstring = bytearray()
    for inside_code in bs:
        if inside_code == 0x0a:
            inside_code = 0x0a
        elif inside_code < 9:
            inside_code = 0x0a
        elif inside_code > 9 and inside_code < 32:
            inside_code = 0x0a
        elif inside_code == 0xa0:
            inside_code = 0x0a
        rstring.append(inside_code)
    return rstring


def findOrNull(sql):
    cursor.execute(sql)
    one = cursor.fetchone()
    res = []
    if len(one) > 0:
        for i in one:
            if type(i) == bytes:
                res.append(strB2Q(i).decode(encoding="utf-8", errors='ignore'))
            else:
                res.append(i)
    return res


@app.route('/', methods=['GET', 'POST'])
def index():
    pid = request.args.get('pid', '')
    if not pid:
        return json.dumps("pid is required.", ensure_ascii=False)

    res = {}
    [res['claim']] = findOrNull(
        "select claim from cn_claim_all where publicid = '%s';" % (pid))
    [res['description']] = findOrNull(
        "select description from cn_description_all where publicid = '%s';" % (pid))
    [res['mainclass'], res['title']] = findOrNull(
        "select mainclass, title from cn_baseinfo_all where publicido = '%s';" % (pid))

    return json.dumps(res, ensure_ascii=False)


if __name__ == '__main__':
    args = get_args_parser()
    app.run(host='0.0.0.0', port=args.port, debug=True)
