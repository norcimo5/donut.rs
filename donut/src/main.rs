use std::thread::sleep;
use std::time::Duration;

fn rotate(mul:i32 , shift:i32 , x: &mut i32, y: &mut i32) {
  let mut swp:i32  = *x;
  *x -= mul * (*y) >> shift;
  *y += mul * swp >> shift;
  swp = 3145728 - *x *  *x - *y * *y >> 11;
  *x = *x * swp >> 10;
  *y = *y * swp >> 10;
}

fn main() {
    let mut b = [32u8; 1760];  // text buffer
    let mut z = [127i8; 1760];  // z buffer
    let (mut sa, mut ca) = (1024, 0);
    let (mut sb, mut cb) = (1024, 0);

    loop {
        // clear the buffers
        b.fill(32);
        z.fill(127);
        let (mut sj, mut cj) = (0, 1024);
        for _j in 0..90 {
            let (mut si, mut ci) = (0, 1024);
            for _i in 0..324 {
                let (r1, r2, k2) = (1, 2048, 5120 * 1024);
                let x0 = r1 * cj + r2;
                let x1 = ci * x0 >> 10;
                let x2 = ca * sj >> 10;
                let x3 = si * x0 >> 10;
                let x4 = r1 * x2 - (sa * x3 >> 10);
                let x5 = sa * sj >> 10;
                let x6 = k2 + r1 * 1024 * x5 + ca * x3;
                let x7 = cj * si >> 10;
                let x = 40 + 30 * (cb * x1 - sb * x4) / x6;
                let y = 12 + 15 * (cb * x4 + sb * x1) / x6;
                let n = (-ca * x7 - cb * ((-sa * x7 >> 10) + x2) - ci * (cj * sb >> 10) >> 10) - x5 >> 7;
                let o:usize  = x as usize  + 80 * y as usize;
                let zz = (x6 - k2) >> 15;
                if y > 0 && y < 22 && x > 0 && x < 80 && zz < z[o] as i32 {
                    z[o] = zz as i8;
                    b[o] = [ 46, 44, 45, 126, 58, 59, 61, 33, 42, 35, 36, 64, ][if n > 0 { n as usize } else { 0 }];
                }

                // rotate i
                rotate(5, 8, &mut ci, &mut si);
            }
            // rotate j
            rotate(9, 7, &mut cj, &mut sj);
        }

        for _k in 0..1761 {
          if _k % 80 == 0 {
            println!();
          } else {
            print!("{}",  b[_k] as char);
          }
        }
        rotate(5, 7, &mut ca, &mut sa);
        rotate(5, 8, &mut cb, &mut sb);
        sleep(Duration::from_micros(15000));
        print!("\x1b\x5b23A");
    }
}
