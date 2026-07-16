fn main() {
    // constants
    const WIDTH: usize = 100;
    const HEIGHT: usize = 50;
    let (R, r, PI) = (20.0, 10.0, std::f32::consts::PI); 
    let (xo, yo) = (WIDTH as f32/2.0,HEIGHT as f32/2.0);        
    let (K1, K2) = (150.0,100.0); // camera z offset, zoom
    let (Lx, Ly, Lz):(f32,f32,f32) = (0.0, -1.0, -1.0);
    let L_magnitude = (Lx.powi(2) + Ly.powi(2) + Lz.powi(2)).sqrt();
    let (mut A, mut B): (f32, f32) = (0.0, 0.0);
    let brightness: Vec<char> =  "░▒▓█".chars().collect();
    //let brightness: Vec<char> =  "▏▎▍▌▋▊▉█".chars().collect();
    //let brightness: Vec<char> =  ".,-~:;=!*#$@@".chars().collect();
    let max_brightness_idx = (brightness.len() - 1) as f32;

    loop{
        let mut frame_buffer = [' '; WIDTH * HEIGHT];
        let mut z_buffer = [0.0f32; WIDTH * HEIGHT];

        let mut theta = 0.0;
        while theta < 2.0*PI { 
            let mut phi = 0.0;
            while phi < 2.0*PI { 
                // surface normals
                let x_sn = theta.sin() * phi.cos(); 
                let y_sn = theta.sin() * phi.sin(); 
                let z_sn = theta.cos(); 
                
                // rotation over x-axis: y,z
                let y_sn1 = y_sn * A.cos() - z_sn * A.sin();
                let z_sn1 = y_sn * A.sin() + z_sn * A.cos();

                // rotation over z-axis: x,y
                let x_sn2 = x_sn * B.cos() - y_sn1 * B.sin();
                let y_sn2 = x_sn * B.sin() + y_sn1 * B.cos();
                // final surface normals: x_sn2,y_sn2,z_sn1
                
                // dot product between light source and surface normals 
                let dot_product = x_sn2*Lx + y_sn2*Ly + z_sn1*Lz;

                // torus coordinates
                let x = (R + r*theta.sin())*phi.cos();
                let y = (R + r*theta.sin())*phi.sin();
                let z = r*theta.cos();

                // rotation over x-axis: y,z
                let y1 = y * A.cos() - z * A.sin();
                let z1 = y * A.sin() + z * A.cos();

                // rotation over z-axis: x,y
                let x2 = x * B.cos() - y1 * B.sin();
                let y2 = x * B.sin() + y1 * B.cos();
                // final coordinates: x2,y2,z1
                
                // projection coordinates
                let ooz = 1.0/(z1+K1);
                let xp = ((K2*ooz*2.0)*x2+xo) as usize; 
                let yp = ((K2*ooz)*y2+yo) as usize;

                let index = yp * WIDTH + xp;
                if xp < WIDTH && yp < HEIGHT {
                    let index = yp * WIDTH + xp; 
                    if ooz > z_buffer[index] {
                        z_buffer[index] = ooz;

                        let normalized_dot = f32::max(0.0, dot_product / L_magnitude);
                        let mut L_idx = (normalized_dot * max_brightness_idx) as usize;
                        L_idx = std::cmp::min(L_idx, brightness.len() - 1);

                        frame_buffer[index]= brightness[L_idx];
                   } 
                }
                phi += 0.1;
            }
            theta += 0.5;
        }
        
        print!("\x1b[H");
        for i in 0..(WIDTH * HEIGHT) { 
            print!("{}", frame_buffer[i]);
            if i%WIDTH == WIDTH-1{
                println!();
            }
        }

        A += 0.02; B += 0.06;
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
