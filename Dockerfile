# 使用Ubuntu 22.04作为基础镜像
FROM ubuntu:22.04

# 设置环境变量，避免在安装过程中交互
ENV DEBIAN_FRONTEND=noninteractive

# 更新包列表并安装git、curl和sudo
RUN apt-get update -y && \
    apt-get install -y git curl sudo && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /root

# 设置默认命令
CMD ["bash"]
