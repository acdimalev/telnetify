#include <assert.h>
#include <stdio.h>

#include <unistd.h>

#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/tcp.h>

void usage() {
}

int main(int argc, char **argv) {
  if (argc < 2) { usage(); return 0; }

  int tcp6_listen_socket;
  uint16_t port;

  { // open tcp socket
    tcp6_listen_socket = socket(AF_INET6, SOCK_STREAM, 0);
    struct sockaddr_in6 sockaddr_in6 = { AF_INET6, 0, 0, IN6ADDR_LOOPBACK_INIT, 0 };

    int retval;

    retval = bind(tcp6_listen_socket, (struct sockaddr *)&sockaddr_in6, sizeof(sockaddr_in6));
    if (retval) { perror(0); }

    retval = listen(tcp6_listen_socket, 0);
    if (retval) { perror(0); }

    // grab port number
    socklen_t addrlen = sizeof(sockaddr_in6);
    retval = getsockname(tcp6_listen_socket, (struct sockaddr *)&sockaddr_in6, &addrlen);
    if (retval) { perror(0); }

    assert(addrlen == sizeof(sockaddr_in6));

    port = ntohs(sockaddr_in6.sin6_port);
  }

  char str_port[7]; // '-' + [0, 32767]

  { // convert port to string for later use
    // dash prefix forces telnet protocol
    int retval = snprintf(str_port, sizeof(str_port), "-%d", port);
    assert(retval > 0);
    assert(retval < sizeof(str_port));
  }

  pid_t pid;

  { // spawn telnet
    pid = fork();
    assert(pid >= 0);

    if (!pid) {
      close(tcp6_listen_socket);
      execl("/usr/bin/telnet", "telnet", "::1", "--", str_port, (char *)0);
      assert(0);
    }
  }

  int tcp6_accept_socket;

  { // wait for telnet to connect
    tcp6_accept_socket = accept(tcp6_listen_socket, 0, 0);
    close(tcp6_listen_socket);
  }

  { // spawn application
    pid = fork();
    assert(pid >= 0);

    if (!pid) {
      // dup tcp6_accept_socket to 0,1,2
      dup2(tcp6_accept_socket, 0);
      close(tcp6_accept_socket);
      dup2(0, 1);
      dup2(0, 2);

      // exec application
      execv(argv[1], &argv[1]);
      assert(0);
    }
  }

  while(wait() != -1) { }

  return 0;
}
