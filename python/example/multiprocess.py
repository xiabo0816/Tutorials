import multiprocessing
from multiprocessing import Pool, TimeoutError
import time
import sys
import traceback
import requests
import re

# 文件编码
FILE_ENCODE = 'UTF-8'

FILELIST = [
    "https://baijiahao.baidu.com/s?id=1718812107504330775",
    "https://baijiahao.baidu.com/s?id=1719167747919266045",
    "https://baijiahao.baidu.com/s?id=1719172487170650376",
    "https://baijiahao.baidu.com/s?id=1719195258394475173"
]

G_FOUT = open("multiprocess.log","w")

def singleprocessing(args):
    r = requests.get(args)
    title = re.findall(r'<title>.*</title>', str(r.content, "UTF-8"))
    if len(title) > 0:
        return title[0]
    else:
        return ""

def _run_pool(args):
    try:
        return singleprocessing(args['filename'])
    except:
        print("Unexpected error:", sys.exc_info(), file=sys.stderr)
        print("Unexpected error:", traceback.format_exc(), file=sys.stderr)
        return {}

def _run_pool_callback(args):
    print(args)
    G_FOUT.write(args+"\n")


if __name__ == '__main__':
    # PROCESSES = multiprocessing.cpu_count()
    PROCESSES = 1

    print('Creating pool with %d processes\n' % PROCESSES, file=sys.stderr)
    e1 = time.time()
    pool = Pool(PROCESSES)
    for i in range(len(FILELIST)):
        param = {'filename': FILELIST[i]}
        pool.apply_async(_run_pool, args=(param, ), callback=_run_pool_callback)
    pool.close()
    pool.join()
    e2 = time.time()
    print(float(e2 - e1), file=sys.stderr)

