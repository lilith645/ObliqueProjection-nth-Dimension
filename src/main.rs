/*
 * main.rs
 * This file is part of Oblique_Projection
 *
 * Copyright (C) 2017 - Lilith Wynter
 *
 * Oblique_Projection is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * Oblique_Projection is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Oblique_Projection. If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]

extern crate glium;
extern crate image;

use std::f64;
use std::fs::File;
use std::path::Path;

#[derive(Copy, Clone)] // Some random thing to define the struct to do stuff? i think?
struct Vertex3f {
  position: [f64; 3],
}

#[derive(Copy, Clone)] // Some random thing to define the struct to do stuff? i think?
struct Vertex2f {
  position: [f64; 2],
}

implement_vertex!(Vertex3f, position);
implement_vertex!(Vertex2f, position);

fn convert_point_3d_to_2d(point: Vec<Vertex3f>) -> Vec<Vertex2f> {
  let angle: f64 = 63.4;
  
  let mut new_point = Vec::new();
  
  for i in point {
     new_point.push( Vertex2f { position: { [i.position[0] + 0.5*i.position[2]*angle.cos(), i.position[1] + 0.5*i.position[2]*angle.sin()] } });
  }
   
  new_point
}

fn main() {    
    let mut point = Vec::new();
    
    point.push(Vertex3f { position: { [-5.0, -5.0,  -5.0] } }); 
    point.push(Vertex3f { position: { [-5.0,  5.0,  -5.0] } });
    point.push(Vertex3f { position: { [ 5.0,  5.0,  -5.0] } });
    point.push(Vertex3f { position: { [ 5.0, -5.0,  -5.0] } });
    
    point.push(Vertex3f { position: { [-5.0, -5.0,   5.0] } });
    point.push(Vertex3f { position: { [-5.0,  5.0,   5.0] } });
    point.push(Vertex3f { position: { [ 5.0,  5.0,   5.0] } });
    point.push(Vertex3f { position: { [ 5.0, -5.0,   5.0] } });
    
    let mut image: Vec<Vertex2f> = convert_point_3d_to_2d(point);
    
    let mut image_lengthx: i64 = 0;
    let mut image_lengthy: i64 = 0;
    
    for i in image.iter_mut() {
      i.position[0] *= 10.0;
      i.position[1] *= 10.0;
      
      if i.position[0] as i64 > image_lengthx {
        image_lengthx = i.position[0] as i64;
      }
      if i.position[1] as i64 > image_lengthy {
        image_lengthy = i.position[1] as i64;
      }
    } 
    
    let image = image;
    
    if image_lengthx < 0 {
      image_lengthx*=-1;
    }
    if image_lengthy < 0 {
      image_lengthy*=-1;
    }
    
    let mut done: bool = false;

    while !done {
      let mut changed: bool = false;
      if image_lengthx%10 != 0 {
        changed = true;
        image_lengthx+=1;
      } 
      if image_lengthy%10 != 0 {
        changed = true;
        image_lengthy+=1;
      }
      
      if changed == false {
        done = true;
      }
    }
    
    let sizex: u32 = image_lengthx as u32 * 2; 
    let sizey: u32 = image_lengthy as u32 * 2;
    
    println!("Image Width: {} Height: {}", sizex, sizey);
    
    let mut imgbuffer = image::ImageBuffer::new(sizex, sizey);
    
    for (x, y, pixel) in imgbuffer.enumerate_pixels_mut() {
      let crnt_px: i64 = (x as i32 - sizex as i32 /2) as i64;
      let crnt_py: i64 = (y as i32 - sizey as i32 /2) as i64;
      
      for i in image.iter() {

        if i.position[0] as i64 == crnt_px {
         if i.position[1] as i64 == crnt_py {
            // Place white pixel
            *pixel = image::Luma([255]);
          }
        }
      }
    }
    
    let ref mut fout = File::create(&Path::new("2DVisualisation.png")).unwrap();
    
    let _ = image::ImageLuma8(imgbuffer).save(fout, image::PNG);
}
