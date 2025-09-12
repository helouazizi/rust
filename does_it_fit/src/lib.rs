pub mod areas_volumes;

pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};


pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {

    // i need to extract the he ares depend on the kind passed as param all area function exist in area volumes and 
    let area = match kind {
        areas_volumes   ::GeometricalShapes::Square =>{
           areas_volumes::square_area(a)
        }
        areas_volumes   ::GeometricalShapes::Rectangle =>{
             areas_volumes::rectangle_area(a,b)
        }
        areas_volumes   ::GeometricalShapes::Circle =>{
             areas_volumes::circle_area(a) as usize
        }
        areas_volumes   ::GeometricalShapes::Triangle =>{
           areas_volumes::triangle_area(a,b) as usize
        }


    };
   

    area * times <=  x*y
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
        let vol = match kind {
        areas_volumes   ::GeometricalVolumes::Cone =>{
           areas_volumes::cone_volume(a , b) as usize
        }
        areas_volumes   ::GeometricalVolumes::Parallelepiped =>{
             areas_volumes::parallelepiped_volume(a,b,c)
        }
        areas_volumes   ::GeometricalVolumes::Sphere =>{
             areas_volumes::sphere_volume(a) as usize
        }
        areas_volumes   ::GeometricalVolumes::Cube =>{
           areas_volumes::cube_volume(a) as usize
        }
         areas_volumes   ::GeometricalVolumes::TriangularPyramid =>{
           areas_volumes::triangular_pyramid_volume(a as f64,b) as usize
        }


    };


    vol * times <=   areas_volumes::parallelepiped_volume(x,y,z)
}



 