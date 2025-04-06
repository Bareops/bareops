FROM debian:latest

# Update packages
RUN apt-get update

# Install OpenSSH server
RUN apt-get install -y openssh-server

# Create directory for SSH server
RUN mkdir /var/run/sshd

# Set password for root user
RUN echo 'root:root123' | chpasswd

# Permit root login via SSH
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# Expose port 22 for SSH
EXPOSE 22

# Command to start SSH server when the container starts
CMD ["/usr/sbin/sshd", "-D"]
