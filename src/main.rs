use std::{ fs::File, io::{ Read, Write }, net::{ TcpListener, TcpStream }, os::raw };
use bmp::{ px, Image, Pixel };

extern "C" {
    pub fn scCreateCamera(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        framerate: f32
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn scDeleteCamera(camera: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn scSendFrame(
        camera: *mut ::std::os::raw::c_void,
        image_bits: *const ::std::os::raw::c_uchar
    );
}
extern "C" {
    pub fn scWaitForConnection(camera: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scIsConnected(camera: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}

unsafe fn handle_client(mut stream: TcpStream) {
    let mut count = 0;
    let cam_width = 1200;
    let cam_height = 1600;
    let cam = scCreateCamera(1200, 1600, 60.0);
    if cam.is_null() {
        println!("failed to create camera");
        return;
    }
    scWaitForConnection(cam);
    if scIsConnected(cam) == 0 {
        println!("failed to connect to client");
        return;
    }
    loop {
        if count > 1000 {
            break;
        }
        const len: usize = 1572 * 4736;
        let mut data = [0 as u8; len];
        let resu = stream.read_exact(&mut data);
        if resu.is_err() {
            println!("Error reading from stream: {:?}", resu.err());
            break;
        }
        let mut im: Image = Image::new(1178, 1572);
        let rowBytes = 4736;
        let mut raw_image = vec![0u8; (cam_width * cam_height * 3) as usize]; // BGR format

        for (x, y) in im.coordinates() {
            let index = (y * rowBytes + x * 4) as usize;
            let r = data[index + 1];
            let g = data[index + 2];
            let b = data[index + 3];
            // Store the image in the top-left corner of the larger raw_image buffer
            let i = ((y * cam_width + x) * 3) as usize; // Index in the larger raw_image buffer

            // Store in BGR order
            raw_image[i] = b;
            raw_image[i + 1] = g;
            raw_image[i + 2] = r;
        }
        scSendFrame(cam, raw_image.as_ptr());

        count += 1;

        // println!("Done with connection");
    }
}

fn main() {
    let listener = TcpListener::bind("192.168.1.123:3333").unwrap();
    println!("Listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                unsafe {
                    handle_client(stream);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    println!("Hello, world!");
}
