FROM debian:latest

# Update packages
RUN apt-get update

# Install OpenSSH server
RUN apt-get install -y openssh-server

# Create directory for SSH server
RUN mkdir /var/run/sshd

COPY test-key.pub /root/.ssh/authorized_keys

# Expose port 22 for SSH
EXPOSE 22

# Command to start SSH server when the container starts
CMD ["/usr/sbin/sshd", "-D"]
