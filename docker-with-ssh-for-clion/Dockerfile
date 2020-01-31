FROM ubuntu:18.04

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    ssh build-essential clang++-9 cmake rsync tar python-clang-9 \
 && apt-get clean

RUN ( \
     echo 'LogLevel DEBUG2'; \
     echo 'PermitRootLogin yes'; \
     echo 'PasswordAuthentication yes'; \
     echo 'Subsystem sftp /usr/lib/openssh/sftp-server'; \
    ) > /etc/ssh/sshd_config \
 && mkdir /run/sshd

RUN useradd -m clion \
 && yes clion | passwd clion

CMD ["/usr/sbin/sshd", "-D", "-e", "-f", "/etc/ssh/sshd_config"]
