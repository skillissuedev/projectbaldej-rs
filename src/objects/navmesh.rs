use glam::Vec2;
use glium::Display;
//use recast_rs::{util, Heightfield, CompactHeightfield, NoRegions, PolyMesh, ContourBuildFlags, ContourSet};
use crate::managers::{debugger, navigation::{self, NavMeshDimensions}, physics::ObjectBodyParameters};
use super::{Object, Transform, ObjectGroup, gen_object_id};

//#[derive(Debug)]
pub struct NavigationGround {
    name: String,
    transform: Transform,
    parent_transform: Option<Transform>,
    children: Vec<Box<dyn Object>>,
    id: u128,
    groups: Vec<ObjectGroup>,
    dimensions: NavMeshDimensions,
    //grid: Grid<Option<()>>
}

impl NavigationGround {
    pub fn new(name: &str, area_size: Vec2) -> Self {
        let x_cells_count = ((area_size.x.round() / 2.0) as i32).abs_diff(0) as i32;
        let z_cells_count = ((area_size.y.round() / 2.0) as i32).abs_diff(0) as i32;

        Self { 
            name: name.into(),
            transform: Transform::default(),
            parent_transform: None,
            children: vec![],
            id: gen_object_id(),
            groups: vec![],
            dimensions: NavMeshDimensions {
                area_size_world: area_size,
                x_cells_count,
                z_cells_count,
                position: Vec2::new(0.0, 0.0),
            },
            //grid: Grid::new(x_cells_count, z_cells_count, Some(())),
        }
    }
}


impl Object for NavigationGround {
    fn start(&mut self) { }

    fn update(&mut self) { 
        let pos = self.global_transform().position;
        self.dimensions.position = Vec2::new(pos.x, pos.z);

        navigation::add_navmesh(*self.object_id(), self.dimensions.clone());
    }

    fn render(&mut self, _display: &mut Display, _target: &mut glium::Frame) { }

    fn children_list(&self) -> &Vec<Box<dyn Object>> {
        &self.children
    }

    fn children_list_mut(&mut self) -> &mut Vec<Box<dyn Object>> {
        &mut self.children
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn object_type(&self) -> &str {
        "EmptyObject"
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn local_transform(&self) -> Transform {
        self.transform
    }



    fn set_local_transform(&mut self, transform: Transform) {
        self.transform = transform
    }

    fn parent_transform(&self) -> Option<Transform> {
        self.parent_transform
    }

    fn set_parent_transform(&mut self, transform: Transform) {
        self.parent_transform = Some(transform);
    }

    fn set_body_parameters(&mut self, _rigid_body: Option<ObjectBodyParameters>) {
        debugger::error("NavMesh object error!\ncan't use set_body_parameters in this type of objects");
    }

    fn body_parameters(&self) -> Option<ObjectBodyParameters> {
        None
    }

    fn object_id(&self) -> &u128 {
        &self.id
    }

    fn groups_list(&mut self) -> &mut Vec<super::ObjectGroup> {
        &mut self.groups
    }

    fn call(&mut self, name: &str, args: Vec<&str>) -> Option<String> {
        if name == "test" {
            println!("test message {}", args[0])
        }
        None
    }
}

impl std::fmt::Debug for NavigationGround {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NavigationGround")
            .field("name", &self.name())
            .field("object_type", &self.object_type())
            .field("children", &self.children_list())
            .finish()
    }
}

enum CurrentAxis {
    X,
    Y,
    Z
}

#[derive(Debug)]
pub enum NavMeshError {
    HeightmapError,
    RasterizeError,
    PolyMeshError
}

