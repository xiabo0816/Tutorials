FROM centos

COPY ./app /app

RUN set -ex \
        && chmod +x /app/run.sh \
        && yum -y install python3 \
        && pip3 install -r /app/requirements.txt  \     
        && rm -rf ~/.cache

# 容器暴露端口
EXPOSE 8080

# 启动命令
ENTRYPOINT ["./run.sh"]

# CMD ["python3"]
