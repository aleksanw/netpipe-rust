use std::io;
use std::net::Shutdown::Write;
use std::net::TcpStream;
use std::thread;

fn main() {
    let mut sock = TcpStream::connect("127.0.0.1:12345").unwrap();

    // Spawn thread sending input to remote:
    {
        let mut sock = sock.try_clone().unwrap();
        thread::spawn(move || {
            // Copy from stdin until EOF.
            io::copy(&mut io::stdin(), &mut sock).unwrap();
            // Send FIN to signal end of data.
            sock.shutdown(Write).unwrap();
        });
    }

    // Let main receving from remote to stdout.
    // Copy to stdout until remote closes.
    io::copy(&mut sock, &mut io::stdout()).unwrap();
    // When main() returns, other threads are terminated as well.
}
