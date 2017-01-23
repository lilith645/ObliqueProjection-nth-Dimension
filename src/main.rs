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
extern crate csv;

use std::f64;
use std::fs::File;
use std::path::Path;
use std::fmt::Display;

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

fn array_to_image(image: &mut Vec<[f64; 2]>) {
    println!("Creating 2D projection");
    
    let mut image_lengthx: i64 = 0;
    let mut image_lengthy: i64 = 0;
    
    for i in image.iter_mut() {
      i[0] *= 10.0;
      i[1] *= 10.0;
      
      if i[0] as i64 > image_lengthx {
        image_lengthx = i[0] as i64;
      }
      if i[1] as i64 > image_lengthy {
        image_lengthy = i[1] as i64;
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
    
    println!("    image Width: {} Height: {}", sizex, sizey);
    
    let mut imgbuffer = image::ImageBuffer::new(sizex, sizey);
    
    for (x, y, pixel) in imgbuffer.enumerate_pixels_mut() {
      let crnt_px: i64 = (x as i32 - sizex as i32 /2) as i64;
      let crnt_py: i64 = (y as i32 - sizey as i32 /2) as i64;
      
      for i in image.iter() {

        if i[0] as i64 == crnt_px {
         if i[1] as i64 == crnt_py {
            // Place white pixel
            *pixel = image::Luma([255]);
          }
        }
      }
    }
    
    let ref mut fout = File::create(&Path::new("2DVisualisation.png")).unwrap();
    
    let _ = image::ImageLuma8(imgbuffer).save(fout, image::PNG);
    
    println!("Projection created!!");
}

fn read_3d_data(file: &str) -> Vec<[f64; 3]> {
    let mut array_data = Vec::new();
    
    let mut rdr = csv::Reader::from_file(file).unwrap();
    for record in rdr.decode() {
        let (x, y, z): (f64, f64, f64) = record.unwrap();
        array_data.push([x as f64, y as f64, z as f64]);
        println!("{}, {}, {}", x, y, z);
    }
    
    array_data
}

fn oblique_projection_from_3d(file: &str) {
  let data = read_3d_data(file);
  
  let angle: f64 = 63.4;
  let mut array_2d = Vec::new();
  
  for position in data {
    let (x, y, z): (f64, f64, f64) = (position[0], position[1], position[2]);
    array_2d.push([x + 0.5*z*angle.cos(), y + 0.5*z*angle.sin()])
  }
  
  array_to_image(&mut array_2d);
}

fn main() {    
    oblique_projection_from_3d("./data/cube.csv");
}

/*
    let mut point = Vec::new();

    point.push([-5.0, -5.0,  -5.0]); 
    point.push([-5.0,  5.0,  -5.0]);
    point.push([ 5.0,  5.0,  -5.0]);
    point.push([ 5.0, -5.0,  -5.0]);
    
    point.push([-5.0, -5.0,   5.0]);
    point.push([-5.0,  5.0,   5.0]);
    point.push([ 5.0,  5.0,   5.0]);
    point.push([ 5.0, -5.0,   5.0]);
    
    
    convert_nth_dim_to_2_dim(point);

fn convert_nth_dim_to_2_dim<Vec: AsRef<[Dimension]>, Dimension: AsRef<[V]>, V: Display>(md_array: Vec) {
  // Each dimension x, y, z, 4th, 5th, 6th etc...
 // println!("{}", md_array.len());
  let md_array = md_array.as_ref();
  print!("\t[");
  for dimension in md_array {
    let dimension = dimension.as_ref();

    // Every variable in each dimension
    for i in dimension {
        print!("{}, ", i);
    }
    println!("; {}]", dimension.len());
  }
  println!("; {}]", md_array.len());
}*/
