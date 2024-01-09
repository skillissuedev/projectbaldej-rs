use crate::managers::{render, physics::ObjectBodyParameters};
use super::{Object, Transform, gen_object_id};

#[derive(Debug)]
pub struct CameraPosition {
    name: String,
    transform: Transform,
    parent_transform: Option<Transform>,
    children: Vec<Box<dyn Object>>,
    body: Option<ObjectBodyParameters>, 
    id: u128
}

impl CameraPosition {
    pub fn new(name: &str) -> Self {
        CameraPosition { transform: Transform::default(), children: vec![], name: name.to_string(), parent_transform: None, body: None, id: gen_object_id() }
    }
}

impl Object for CameraPosition {
    fn start(&mut self) { }

    fn update(&mut self) {
        let global_transform = self.get_global_transform();
        render::set_camera_position(global_transform.position);
        render::set_camera_rotation(global_transform.rotation);
    }


    fn render(&mut self, _display: &mut glium::Display, _target: &mut glium::Frame) { }

    fn get_children_list(&self) -> &Vec<Box<dyn Object>> {
        &self.children
    }

    fn get_children_list_mut(&mut self) -> &mut Vec<Box<dyn Object>> {
        &mut self.children
    }



    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_object_type(&self) -> &str {
        "CameraPosition"
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn get_local_transform(&self) -> Transform {
        self.transform
    }

    fn set_local_transform(&mut self, transform: Transform) {
        self.transform = transform
    }

    fn get_parent_transform(&self) -> Option<Transform> {
        self.parent_transform
    }

    fn set_parent_transform(&mut self, transform: Transform) {
        self.parent_transform = Some(transform);
    }

    fn set_body_parameters(&mut self, rigid_body: Option<ObjectBodyParameters>) {
        self.body = rigid_body
    }

    fn get_body_parameters(&self) -> Option<ObjectBodyParameters> {
        self.body
    }

    fn get_object_id(&self) -> &u128 {
        &self.id
    }

    fn call(&mut self, name: &str, args: Vec<&str>) -> Option<String> {
        None
    }
}
